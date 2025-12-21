use immargs::args;
use std::ffi::OsString;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[test]
fn type_none() {
    args! {
        --flag,
    }

    let args = Args::from(["test", "--flag"]);
    assert!(args.flag);
}

#[test]
fn type_bool() {
    args! {
        --value <value> bool,
    }

    let args = Args::from(["test", "--value", "true"]);
    assert!(args.value.unwrap());
}

#[test]
fn type_char() {
    args! {
        --value <value> char,
    }

    let args = Args::from(["test", "--value", "X"]);
    assert!(args.value.unwrap() == 'X');
}

#[test]
fn type_u8() {
    args! {
        --value <value> u8,
    }

    let args = Args::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u16() {
    args! {
        --value <value> u16,
    }

    let args = Args::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u32() {
    args! {
        --value <value> u32,
    }

    let args = Args::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u64() {
    args! {
        --value <value> u64,
    }

    let args = Args::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_u128() {
    args! {
        --value <value> u128,
    }

    let args = Args::from(["test", "--value", "47"]);
    assert!(args.value.unwrap() == 47);
}

#[test]
fn type_i8() {
    args! {
        --value <value> i8,
    }

    let args = Args::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i16() {
    args! {
        --value <value> i16,
    }

    let args = Args::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i32() {
    args! {
        --value <value> i32,
    }

    let args = Args::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i64() {
    args! {
        --value <value> i64,
    }

    let args = Args::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_i128() {
    args! {
        --value <value> i128,
    }

    let args = Args::from(["test", "--value", "-47"]);
    assert!(args.value.unwrap() == -47);
}

#[test]
fn type_f32() {
    args! {
        --value <value> f32,
    }

    let args = Args::from(["test", "--value", "1.25"]);
    assert!(args.value.unwrap() == 1.25);
}

#[test]
fn type_f64() {
    args! {
        --value <value> f64,
    }

    let args = Args::from(["test", "--value", "1.25"]);
    assert!(args.value.unwrap() == 1.25);
}

#[test]
fn type_string() {
    args! {
        --value <value> String,
    }

    let args = Args::from(["test", "--value", "hello"]);
    assert!(args.value.unwrap() == "hello");
}

#[test]
fn type_os_string() {
    args! {
        --value <value> OsString,
    }

    let args = Args::from(["test", "--value", "hello"]);
    assert!(args.value.unwrap() == "hello");
}

#[test]
fn type_pathbuf() {
    args! {
        --value <value> PathBuf,
    }

    let args = Args::from(["test", "--value", "hello"]);
    assert!(args.value.unwrap().to_str().unwrap() == "hello");
}

#[test]
fn type_ipv4addr() {
    args! {
        --value <value> Ipv4Addr,
    }

    let args = Args::from(["test", "--value", "127.0.0.1"]);
    assert!(args.value.unwrap() == Ipv4Addr::new(127, 0, 0, 1));
}
