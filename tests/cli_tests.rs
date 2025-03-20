use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("target");
    path.push("debug");
    path.push("port-claim");
    path
}

fn get_package_version() -> String {
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let version_line = cargo_toml
        .lines()
        .find(|line| line.trim().starts_with("version = "))
        .expect("Could not find version in Cargo.toml");

    let version = version_line
        .split('"')
        .nth(1)
        .expect("Invalid version format in Cargo.toml");

    version.to_string()
}

#[test]
fn test_version_flag() {
    let output = Command::new(get_binary_path())
        .args(["--version"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let expected_version = get_package_version();

    assert!(
        stdout.contains(&format!("port-claim {}", expected_version)),
        "Version output incorrect"
    );
}

#[test]
fn test_help_flag() {
    let output = Command::new(get_binary_path())
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage:"), "Help output missing usage");
    assert!(
        stdout.contains("-v, --verbose"),
        "Help output missing verbose flag"
    );
}

#[test]
fn test_no_args_shows_error() {
    let output = Command::new(get_binary_path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Command should fail without args");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("No ports specified"),
        "Error message incorrect"
    );
}

#[test]
fn test_invalid_port() {
    let output = Command::new(get_binary_path())
        .args(["invalid_port"])
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command should fail with invalid port"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("not a valid port number"),
        "Error message incorrect"
    );
}

#[test]
fn test_verbose_output_free_port() {
    // Choose a likely free port
    let port = 60123;

    let output = Command::new(get_binary_path())
        .args([port.to_string().as_str(), "-v"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    // If port is free, it will say "Port x is available"
    // If port is in use, it will say "Port x is in use" followed by "killed"
    assert!(
        stdout.contains(&format!("Port {} is", port)),
        "Verbose output incorrect"
    );
}

#[test]
fn test_port_detection() {
    // Just test that port-claim can detect a free port correctly
    // This avoids testing the "kill" functionality which kills our test process

    // Use a high port number that's unlikely to be in use
    let port = 62345;

    let output = Command::new(get_binary_path())
        .args([port.to_string().as_str(), "-v"])
        .output()
        .expect("Failed to execute command");

    // Only check if it produces output about the port
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(&format!("Port {} is", port)),
        "Missing port status in output"
    );
}
