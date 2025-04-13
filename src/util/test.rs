#![cfg(all(std, test))]

const FIREHAZARD_TEST_FLAVOR : &'static str = "FIREHAZARD_TEST_FLAVOR";



#[cfg(nope)] // TODO: parse stdout
/// Runs `target\...\firehazard-{...}.exe --list` and returns it's error code
pub fn list() -> impl Iterator<Item = std::string::String> {
    let exe = std::env::current_exe().expect("unable to determine test executable path");
    let output = std::process::Command::new(exe).arg("--list").output().expect("unable to launch test executable");
    let code = output.status.code().expect("test executable was killed by signal");
    assert_eq!(code, 0, "test executable failed to list tests");
    let stdout = std::string::String::from_utf8_lossy(&output.stdout);
    panic!("{stdout:?}");
    stdout.split('\n').map(|line| line.trim().into())
}

/// Runs `target\...\firehazard-{...}.exe --include-ignored --exact {test}` and returns it's error code
pub fn run_one_exact_flavor(test: &str, flavor: &str) -> Option<i32> {
    let exe = std::env::current_exe().expect("unable to determine test executable path");
    let mut cmd = std::process::Command::new(exe);
    cmd.args(["--include-ignored", "--exact", test]);
    cmd.env(FIREHAZARD_TEST_FLAVOR, flavor);
    cmd.stderr(std::process::Stdio::null()); // XXX: printing test output really wrecks output... but so does this
    cmd.stdout(std::process::Stdio::null()); // XXX: printing test output really wrecks output... but so does this
    cmd.status().expect("unable to launch test executable").code()
}

pub fn flavor() -> std::string::String { std::env::var(FIREHAZARD_TEST_FLAVOR).unwrap_or_default() }
