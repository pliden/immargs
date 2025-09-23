use immargs::immargs;
use std::ffi::OsString;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[test]
fn type_none() {
    immargs! {
        --flag,
    }

    let args = ImmArgs::from(["test", "--flag"]);
    assert!(args.flag);
}

#[test]
fn type_bool() {
    immargs! {
        --value <value> bool,
    }

    let args = ImmArgs::from(["test", "--value", "true"]);
    assert!(args.value.unwrap());
}

#[test]
fn type_char() {
    immargs! {
        --value <value> char,
    }

    let args = ImmArgs::from(["test", "--value", "X"]);
    assert!(args.value.unwrap() == 'X');
}

#[test]
fn type_u8() {
    immargs! {
        --value <value> u8,
    }

    let args = ImmArgs::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u16() {
    immargs! {
        --value <value> u16,
    }

    let args = ImmArgs::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u32() {
    immargs! {
        --value <value> u32,
    }

    let args = ImmArgs::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u64() {
    immargs! {
        --value <value> u64,
    }

    let args = ImmArgs::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u128() {
    immargs! {
        --value <value> u128,
    }

    let args = ImmArgs::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_i8() {
    immargs! {
        --value <value> i8,
    }

    let args = ImmArgs::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i16() {
    immargs! {
        --value <value> i16,
    }

    let args = ImmArgs::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i32() {
    immargs! {
        --value <value> i32,
    }

    let args = ImmArgs::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i64() {
    immargs! {
        --value <value> i64,
    }

    let args = ImmArgs::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i128() {
    immargs! {
        --value <value> i128,
    }

    let args = ImmArgs::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_f32() {
    immargs! {
        --value <value> f32,
    }

    let args = ImmArgs::from(["test", "--value", "1.25"]);
    assert!(args.value.unwrap() == 1.25);
}

#[test]
fn type_f64() {
    immargs! {
        --value <value> f64,
    }

    let args = ImmArgs::from(["test", "--value", "1.25"]);
    assert!(args.value.unwrap() == 1.25);
}

#[test]
fn type_string() {
    immargs! {
        --value <value> String,
    }

    let args = ImmArgs::from(["test", "--value", "hello"]);
    assert!(args.value.unwrap() == "hello");
}

#[test]
fn type_os_string() {
    immargs! {
        --value <value> OsString,
    }

    let args = ImmArgs::from(["test", "--value", "hello"]);
    assert!(args.value.unwrap() == "hello");
}

#[test]
fn type_pathbuf() {
    immargs! {
        --value <value> PathBuf,
    }

    let args = ImmArgs::from(["test", "--value", "hello"]);
    assert!(args.value.unwrap().to_str().unwrap() == "hello");
}

#[test]
fn type_ipv4addr() {
    immargs! {
        --value <value> Ipv4Addr,
    }

    let args = ImmArgs::from(["test", "--value", "127.0.0.1"]);
    assert!(args.value.unwrap() == Ipv4Addr::new(127, 0, 0, 1));
}
