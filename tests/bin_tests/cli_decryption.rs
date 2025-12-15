// 2025 Steven Chiacchira
use assert_cmd::cargo;
use assert_fs::{fixture::PathChild, TempDir};
use std::fs;
use std::path;

#[test]
fn no_such_file() {
    fs::exists("_.enc").expect("File _.enc should not exist");

    let mut command = cargo::cargo_bin_cmd!("decrypt");

    let input_file_dir = TempDir::new().unwrap();
    let input_file = input_file_dir.child("_.enc");

    command
        .arg("--key")
        .arg("42")
        .arg(input_file.path())
        .arg("_.txt");

    command
        .assert()
        .failure()
        .stderr(predicates::str::contains("FileReadError"));
}

#[test]
fn no_key_provided() {
    let mut command = cargo::cargo_bin_cmd!("decrypt");

    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01_k0.enc");
    let output_file_dir = TempDir::new().unwrap();
    let output_file = output_file_dir.child("_.txt");

    command.arg(message_file).arg(output_file.path());

    command.assert().failure().stderr(predicates::str::contains(
        "the following required arguments were not provided",
    ));
}

#[test]
fn numeric_keys() {
    let expected_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let expected_message =
        fs::read(expected_file).expect("Could not find plaintext in data directory");

    for key in 0..3 {
        let encrypted_file =
            env!("CARGO_MANIFEST_DIR").to_owned() + &format!("/data/tests/text_01_k{}.enc", key);
        let encrypted_file = path::Path::new(&encrypted_file);

        let output_file_dir = TempDir::new().unwrap();
        let output_file = output_file_dir.child("output.txt");

        let mut command = cargo::cargo_bin_cmd!("decrypt");
        command
            .arg("--key")
            .arg(key.to_string())
            .arg(encrypted_file)
            .arg(output_file.path());
        command
            .assert()
            .success()
            .stderr(predicates::str::contains("Finished"));

        let decrypted_message = fs::read(&output_file).unwrap();
        assert!(expected_message == decrypted_message[..expected_message.len()]);

        output_file_dir.close().unwrap();
    }
}

#[test]
fn str_keys() {
    let expected_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");
    let expected_message =
        fs::read(expected_file).expect("Could not find plaintext in data directory");

    for key in ["Foo", "Bar"] {
        let encrypted_file =
            env!("CARGO_MANIFEST_DIR").to_owned() + &format!("/data/tests/text_01_k{}.enc", key);
        let encrypted_file = path::Path::new(&encrypted_file);

        let output_file_dir = TempDir::new().unwrap();
        let output_file = output_file_dir.child("output.txt");

        let mut command = cargo::cargo_bin_cmd!("decrypt");
        command
            .arg("--key")
            .arg(key)
            .arg(encrypted_file)
            .arg(output_file.path());
        command
            .assert()
            .success()
            .stderr(predicates::str::contains("Finished"));

        let decrypted_message = fs::read(&output_file).unwrap();
        assert!(expected_message == decrypted_message[..expected_message.len()]);

        output_file_dir.close().unwrap();
    }
}
