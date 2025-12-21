use immargs::Error;
use immargs::args;

#[test]
fn version() {
    args! {
        -v --version,
    }

    let result = ImmArgs::try_from_raw(["test", "-v"]);
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    assert!(matches!(result, Err(Error::Version { message}) if message == version ));
}
