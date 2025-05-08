use std::ffi::{CStr, CString};
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

pub fn metaphone3(word: &str, encode_vowels: bool, encode_exact: bool) -> (String, String) {
    let c_word = CString::new(word).expect("CString::new failed");
    unsafe {
        let result = EncodeMetaphone(c_word.as_ptr(), encode_vowels, encode_exact);

        let primary = CStr::from_ptr(result.primary)
            .to_string_lossy()
            .into_owned();
        let secondary = CStr::from_ptr(result.secondary)
            .to_string_lossy()
            .into_owned();

        FreeMetaphoneResult(result);

        (primary, secondary)
    }
}
#[cfg(test)]
mod tests {
    use crate::metaphone3;

    #[test]
    fn it_works() {
        assert_eq!(
            ("SM0".into(), "XMT".into()),
            metaphone3("SMITH", false, false)
        );
        assert_eq!(
            ("SMA0".into(), "XMAT".into()),
            metaphone3("SMITH", true, false)
        );
        assert_eq!(
            ("KAPLAN".into(), "".into()),
            metaphone3("GOBLIN", true, false)
        );
        assert_eq!(
            ("GABLAN".into(), "".into()),
            metaphone3("GOBLIN", true, true)
        );
    }
}
