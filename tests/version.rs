use immargs::Error;
use immargs::immargs;

#[test]
fn version() {
    immargs! {
        -v --version,
    }

    let result = ImmArgs::try_from(["test", "-v"]);
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    assert!(matches!(result, Err(Error::Version { message}) if message == version ));
}
