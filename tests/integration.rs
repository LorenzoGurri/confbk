extern crate assert_cmd;
extern crate escargot;
extern crate lazy_static;

// TODO: USE THIS
extern crate tempdir;

use assert_cmd::prelude::*;
use escargot::CargoRun;
use lazy_static::lazy_static;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempdir::TempDir;

static CURRENT_DIR: &'static str = "tests/";
lazy_static! {
    static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
        .bin("confbk")
        .current_release()
        .run()
        .unwrap();
}

fn confbk(path: &str) -> Command {
    let mut cmd = CARGO_RUN.command();
    cmd.current_dir(format!("{}", path));
    cmd
}

fn setup_env(tmp_dir: &TempDir) {
    // Files
    fs::File::create(tmp_dir.path().join("backMeUp1")).expect("Failed to create backMeUp1");
    fs::File::create(tmp_dir.path().join("backMeUp2")).expect("Failed to create backMeUp2");
    fs::File::create(tmp_dir.path().join("backMeUp3")).expect("Failed to create backMeUp3");
    fs::File::create(tmp_dir.path().join("\x1B00D8\x1B00FB\x1B0226"))
        .expect("Failed to create file with unicode characters in it");
    // Lists of Files
    let mut list_of_configs1_2 = fs::File::create(tmp_dir.path().join("listOfConfigs1-2"))
        .expect("Failed to create listOfConfigs1-2");
    let mut list_that_fails = fs::File::create(tmp_dir.path().join("listThatFails"))
        .expect("Failed to create listThatFails");

    // Writing filenames to lists
    writeln!(list_of_configs1_2, "backMeUp1\nbackMeUp2")
        .expect("Failed to write to listOfConfigs1-2");
    writeln!(list_that_fails, "IDontExist").expect("Failed to write to listThatFails");
}

#[test]
fn no_args() {
    let currnet = env::current_dir()
        .expect("Could not open current directory")
        .display()
        .to_string();
    confbk(&currnet).assert().failure();
}

#[test]
fn list() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "list").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("backMeUp1")
        .arg("backMeUp2")
        .assert()
        .success()
        .stdout("Backing up\n");
    let backup_dir = format!("{}/confbk_backup", tmp_dir.path().display());
    let dir = PathBuf::from(&backup_dir);
    if dir.is_dir() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| {
                f.expect("Failed to get DirEntry")
                    .path()
                    .file_stem()
                    .expect("Failed to get file_stem")
                    .to_os_string()
            })
            .collect();
        assert!(files.contains(&OsString::from("backMeUp1")));
        assert!(files.contains(&OsString::from("backMeUp2")));
    } else {
        assert!(dir.is_dir());
    }
}

#[test]
fn file() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "file").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-f")
        .arg("listOfConfigs1-2")
        .assert()
        .success()
        .stdout("Backing up\n");
    let backup_dir = format!("{}/confbk_backup", tmp_dir.path().display().to_string());
    let dir = PathBuf::from(&backup_dir);
    if dir.is_dir() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| {
                f.expect("Failed to get DirEntry")
                    .path()
                    .file_stem()
                    .expect("Failed to get file_stem")
                    .to_os_string()
            })
            .collect();
        assert!(files.contains(&OsString::from("backMeUp1")));
        assert!(files.contains(&OsString::from("backMeUp2")));
    } else {
        assert!(dir.is_dir());
    }
}

#[test]
fn tar() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "tar").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-f")
        .arg("listOfConfigs1-2")
        .arg("-t")
        .arg("-o")
        .arg("conf")
        .assert()
        .success()
        .stdout("Backing up\n");
    let backup_file = format!("{}/conf.tar.xz", tmp_dir.path().display().to_string());
    let file = PathBuf::from(&backup_file);
    if file.is_file() {
        let output = Command::new("tar")
            .arg("-tf")
            .arg(&backup_file)
            .output()
            .expect("tar failed to execute");
        let output = String::from_utf8(output.stdout).expect("failed to convert u8 vec to string");
        if !(output.contains("backMeUp1") && output.contains("backMeUp2")) {
            assert!(output.contains("backMeUp1") && output.contains("backMeUp2"));
        }
    } else {
        assert!(file.is_file());
    }
}

#[test]
fn list_and_file() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "list_and_file").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-f")
        .arg("listOfConfigs1-2")
        .arg("-l")
        .arg("backMeUp3")
        .assert()
        .success()
        .stdout("Backing up\n");
    let backup_dir = format!("{}/confbk_backup", tmp_dir.path().display().to_string());
    let dir = PathBuf::from(&backup_dir);
    if dir.exists() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| f.unwrap().path().file_stem().unwrap().to_os_string())
            .collect();

        assert!(files.contains(&OsString::from("backMeUp1")));
        assert!(files.contains(&OsString::from("backMeUp2")));
        assert!(files.contains(&OsString::from("backMeUp3")));
    } else {
        assert!(dir.exists());
    }
}

#[test]
fn dry_run() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "dry_run").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("backMeUp1")
        .arg("-d")
        .assert()
        .success()
        .stdout(
            "Files to be backed up:\n\
             \u{0020}   backMeUp1\n",
        );
}

#[test]
fn verbose() {
    let stdout = "[Debug] Opt {\n    \
                  out: None,\n    \
                  dry_run: true,\n    \
                  quiet: false,\n    \
                  verbose: true,\n    \
                  file: None,\n    \
                  list: [\n        \
                  \"backMeUp1\"\n    \
                  ],\n    \
                  tar: false\n\
                  }\n\
                  Files to be backed up:\n    \
                  backMeUp1\n";
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "verbose").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("backMeUp1")
        .arg("-d")
        .arg("-v")
        .assert()
        .success()
        .stdout(stdout);
}

#[test]
fn quiet() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "quiet").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("backMeUp1")
        .arg("-d")
        .arg("-q")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn quiet_and_verbose() {
    let tmp_dir =
        TempDir::new_in(CURRENT_DIR, "quiet_and_verbose").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("backMeUp1")
        .arg("-v")
        .arg("-q")
        .assert()
        .failure();
}

#[test]
fn file_in_list_doesnt_exist() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "file_in_list_doesnt_exist")
        .expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("IDontExist")
        .assert()
        .failure();
}

#[test]
fn file_doesnt_exist() {
    let tmp_dir =
        TempDir::new_in(CURRENT_DIR, "file_doesnt_exist").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-f")
        .arg("listOfNonExistentConfig")
        .assert()
        .failure();
}

#[test]
fn file_in_file_doesnt_exist() {
    let tmp_dir = TempDir::new_in(CURRENT_DIR, "file_in_file_doesnt_exist")
        .expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-f")
        .arg("listThatFails")
        .assert()
        .failure();
}

#[test]
fn unicode_support() {
    let tmp_dir =
        TempDir::new_in(CURRENT_DIR, "unicode_support").expect("Failed to create tmp dir");
    setup_env(&tmp_dir);
    confbk(&tmp_dir.path().display().to_string())
        .arg("-l")
        .arg("\x1B00D8\x1B00FB\x1B0226")
        .assert()
        .success();
    let backup_dir = format!("{}/confbk_backup", tmp_dir.path().display().to_string());
    let dir = PathBuf::from(&backup_dir);
    if dir.is_dir() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| {
                f.expect("Failed to get DirEntry")
                    .path()
                    .file_stem()
                    .expect("Failed to get file_stem")
                    .to_os_string()
            })
            .collect();
        assert!(files.contains(&OsString::from("\x1B00D8\x1B00FB\x1B0226")));
    } else {
        assert!(dir.is_dir());
    }
}
