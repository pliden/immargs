#![cfg(false)]
use immargs::immargs;
use std::ffi::OsString;

#[test]
fn unicode_invalid() {
    fn invalid_unicode() -> OsString {
        #[cfg(unix)]
        {
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;
            let source = [0x66, 0x6f, 0x80, 0x6f];
            OsStr::from_bytes(&source).to_os_string()
        }
        #[cfg(windows)]
        {
            use std::os::windows::prelude::*;
            let source = [0x0066, 0x006f, 0xD800, 0x006f];
            OsString::from_wide(&source)
        }
    }

    let invalid_arg = invalid_unicode();

    immargs! {
        <value> String,
    }

    let args = ImmArgs::from([OsString::from("test"), invalid_arg]);
    assert!(args.value == "fo�o");
}
