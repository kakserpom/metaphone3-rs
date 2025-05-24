#![warn(clippy::pedantic)]

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;

#[repr(C)]
pub struct MetaphoneResult {
    pub primary: *mut c_char,
    pub secondary: *mut c_char,
}

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
#[cfg(test)]
mod tests {
    use crate::metaphone3;

    #[test]
    fn it_works() {
        assert_eq!(
            ("SM0".into(), "XMT".into()),
            metaphone3("SMITH", false, false).unwrap()
        );
        assert_eq!(
            ("SMA0".into(), "XMAT".into()),
            metaphone3("SMITH", true, false).unwrap()
        );
        assert_eq!(
            ("KAPLAN".into(), "".into()),
            metaphone3("GOBLIN", true, false).unwrap()
        );
        assert_eq!(
            ("GABLAN".into(), "".into()),
            metaphone3("GOBLIN", true, true).unwrap()
        );
    }
}
