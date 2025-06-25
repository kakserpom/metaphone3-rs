#![warn(clippy::pedantic)]

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct MetaphoneResult {
    pub primary: *mut c_char,
    pub secondary: *mut c_char,
}

#[cfg(target_os = "windows")]
mod impl_windows {
    use super::*;
    use libloading::os::windows::{Library, Symbol};
    use std::sync::OnceLock;

    pub struct MetaphoneLib {
        _lib: Library, // чтобы symbols не инвалидировались
        pub encode: Symbol<unsafe extern "C" fn(*const c_char, bool, bool) -> MetaphoneResult>,
        pub free: Symbol<unsafe extern "C" fn(MetaphoneResult)>,
    }

    static LIB: OnceLock<MetaphoneLib> = OnceLock::new();

    pub fn load_library() -> &'static MetaphoneLib {
        LIB.get_or_init(|| {
            let lib =
                unsafe { Library::new("metaphone3.dll") }.expect("Failed to load metaphone3.dll");

            unsafe {
                let encode: Symbol<
                    unsafe extern "C" fn(*const c_char, bool, bool) -> MetaphoneResult,
                > = lib
                    .get(b"EncodeMetaphone\0")
                    .expect("missing EncodeMetaphone");

                let free: Symbol<unsafe extern "C" fn(MetaphoneResult)> = lib
                    .get(b"FreeMetaphoneResult\0")
                    .expect("missing FreeMetaphoneResult");

                MetaphoneLib {
                    _lib: lib,
                    encode,
                    free,
                }
            }
        })
    }
}

// переэкспорт
#[cfg(target_os = "windows")]
use impl_windows::load_library;

/// Safe wrapper
#[cfg(target_os = "windows")]
pub fn metaphone3(
    input: &str,
    encode_vowels: bool,
    encode_exact: bool,
) -> Result<(String, String), NulError> {
    let lib = load_library();
    let c_input = CString::new(input).expect("CString conversion failed");

    let result = unsafe { (lib.encode)(c_input.as_ptr(), encode_vowels, encode_exact) };

    let primary = unsafe { CStr::from_ptr(result.primary) }
        .to_string_lossy()
        .into_owned();
    let secondary = unsafe { CStr::from_ptr(result.secondary) }
        .to_string_lossy()
        .into_owned();

    unsafe {
        (lib.free)(result);
    }

    Ok((primary, secondary))
}

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
    pub fn EncodeMetaphone(
        word: *const c_char,
        encode_vowels: bool,
        encode_exact: bool,
    ) -> MetaphoneResult;
    pub fn FreeMetaphoneResult(result: MetaphoneResult);
}
/// Encode a word into a pair of Metaphone3 encodings
///
/// # Errors
///
/// Will return `NulError` if `word` contains a NUL-byte.
#[cfg(not(target_os = "windows"))]
pub fn metaphone3(
    word: &str,
    encode_vowels: bool,
    encode_exact: bool,
) -> Result<(String, String), NulError> {
    let c_word = CString::new(word)?;
    unsafe {
        let result = EncodeMetaphone(c_word.as_ptr(), encode_vowels, encode_exact);

        let primary = CStr::from_ptr(result.primary)
            .to_string_lossy()
            .into_owned();
        let secondary = CStr::from_ptr(result.secondary)
            .to_string_lossy()
            .into_owned();

        FreeMetaphoneResult(result);

        Ok((primary, secondary))
    }
}
