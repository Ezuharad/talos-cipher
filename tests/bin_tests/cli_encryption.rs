// 2025 Steven Chiacchira
use crate::bin_tests::file_utils::file_contents_equal;
use assert_cmd::cargo;
use assert_fs::{fixture::PathChild, TempDir};
use std::fs;
use std::path;

#[test]
fn no_such_file() {
    fs::exists("_.txt").expect("File _.txt should not exist");

    let mut command = cargo::cargo_bin_cmd!("encrypt");

    let output_file_dir = TempDir::new().unwrap();
    let output_file = output_file_dir.child("_.enc");

    command.arg("_.txt").arg(output_file.path());

    command
        .assert()
        .failure()
        .stderr(predicates::str::contains("FileReadError"));
}

#[test]
fn numeric_keys() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");

    for key in 0..3 {
        let output_file_dir = TempDir::new().unwrap();
        let output_file = output_file_dir.child("output.enc");

        let mut command = cargo::cargo_bin_cmd!("encrypt");
        command
            .arg("--key")
            .arg(key.to_string())
            .arg(message_file)
            .arg(output_file.path());
        command
            .assert()
            .success()
            .stderr(predicates::str::contains("Finished"));

        let expected_file =
            env!("CARGO_MANIFEST_DIR").to_owned() + &format!("/data/tests/text_01_k{}.enc", key);
        let expected_file = path::Path::new(&expected_file);

        let is_equal =
            file_contents_equal(&output_file, expected_file).expect("Missing expected file");
        assert!(is_equal);
        output_file_dir.close().unwrap();
    }
}

#[test]
fn str_keys() {
    let message_file = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tests/text_01.txt");

    for key in ["Foo", "Bar"] {
        let output_file_dir = TempDir::new().unwrap();
        let output_file = output_file_dir.child("output.enc");

        let mut command = cargo::cargo_bin_cmd!("encrypt");
        command
            .arg("--key")
            .arg(key)
            .arg(message_file)
            .arg(output_file.path());
        command
            .assert()
            .success()
            .stderr(predicates::str::contains("Finished"));

        let expected_file =
            env!("CARGO_MANIFEST_DIR").to_owned() + &format!("/data/tests/text_01_k{}.enc", key);
        let expected_file = path::Path::new(&expected_file);

        let is_equal =
            file_contents_equal(&output_file, expected_file).expect("Missing expected file");
        assert!(is_equal);
        output_file_dir.close().unwrap();
    }
}
