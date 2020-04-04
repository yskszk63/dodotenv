use std::process::Command;

#[test]
fn print_usage() {
    let ok = Command::new("target/debug/dodotenv")
        .env_clear()
        .status()
        .unwrap()
        .success();
    assert!(!ok)
}

#[test]
fn exit0() {
    let ok = Command::new("target/debug/dodotenv")
        .args(&["/usr/bin/true"])
        .env_clear()
        .status()
        .unwrap()
        .success();
    assert!(ok)
}

#[test]
fn exitne0() {
    let ok = Command::new("target/debug/dodotenv")
        .args(&["/usr/bin/false"])
        .env_clear()
        .status()
        .unwrap()
        .success();
    assert!(!ok)
}

#[test]
fn nodotenv() {
    let output = Command::new("../../target/debug/dodotenv")
        .args(&["/usr/bin/env"])
        .env_clear()
        .current_dir("tests/no_dotenv")
        .output()
        .unwrap();
    assert_eq!(&output.stdout, b"")
}

#[test]
fn readdotenv() {
    let output = Command::new("../../target/debug/dodotenv")
        .args(&["/usr/bin/env"])
        .env_clear()
        .current_dir("tests/has_dotenv")
        .output()
        .unwrap();
    assert_eq!(&output.stdout, b"FOO=BAR\n")
}

#[test]
fn overrideenv() {
    let output = Command::new("../../target/debug/dodotenv")
        .args(&["/usr/bin/env"])
        .env_clear()
        .env("FOO", "HOGE")
        .current_dir("tests/has_dotenv")
        .output()
        .unwrap();
    assert_eq!(&output.stdout, b"FOO=HOGE\n")
}
