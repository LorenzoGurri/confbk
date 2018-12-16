extern crate assert_cmd;
extern crate escargot;
extern crate lazy_static;

use assert_cmd::prelude::*;
use chrono::Local;
use escargot::CargoRun;
use lazy_static::lazy_static;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

static CURRENT_DIR: &'static str = "tests/environments/";
lazy_static! {
    static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
        .bin("confbk")
        .current_release()
        .run()
        .unwrap();
}

fn confbk(environment: &str) -> Command {
    let mut cmd = CARGO_RUN.command();
    cmd.current_dir(format!("{}{}", CURRENT_DIR, environment));
    cmd
}

#[test]
fn no_args() {
    confbk("").assert().failure();
}

#[test]
fn list() {
    confbk("list_env")
        .arg("-l")
        .arg("backMeUp1")
        .arg("backMeUp2")
        .assert()
        .success()
        .stdout("Backing up\n");
    let time = Local::now().format("%Y_%m_%d");
    let backup_dir = format!("{}list_env/confbk-{}", CURRENT_DIR, time);
    let dir = PathBuf::from(&backup_dir);
    if dir.exists() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| f.unwrap().path().file_stem().unwrap().to_os_string())
            .collect();

        let file1: &OsString = files.get(0).unwrap();
        let file2: &OsString = files.get(1).unwrap();
        assert_eq!(file1, "backMeUp1");
        assert_eq!(file2, "backMeUp2");
        fs::remove_dir_all(backup_dir).expect("Couldn't remove directory");
    } else {
        assert!(dir.exists());
    }
}

#[test]
fn file() {
    confbk("file_env")
        .arg("-f")
        .arg("listOfConfigs1-2")
        .assert()
        .success()
        .stdout("Backing up\n");
    let time = Local::now().format("%Y_%m_%d");
    let backup_dir = format!("{}file_env/confbk-{}", CURRENT_DIR, time);
    let dir = PathBuf::from(&backup_dir);
    if dir.exists() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| f.unwrap().path().file_stem().unwrap().to_os_string())
            .collect();

        let file1: &OsString = files.get(0).unwrap();
        let file2: &OsString = files.get(1).unwrap();
        assert_eq!(file1, "backMeUp1");
        assert_eq!(file2, "backMeUp2");
        fs::remove_dir_all(backup_dir).expect("Couldn't remove directory");
    } else {
        assert!(dir.exists());
    }
}

#[test]
fn tar() {
    confbk("tar_env")
        .arg("-f")
        .arg("listOfConfigs1-2")
        .arg("-x")
        .assert()
        .success()
        .stdout("Backing up\n");
    let time = Local::now().format("%Y_%m_%d");
    let backup_file = format!("{}tar_env/confbk-{}.tar.xz", CURRENT_DIR, time);
    let file = PathBuf::from(&backup_file);
    if file.exists() {
        Command::new("tar")
            .arg("-tf")
            .arg(&backup_file)
            .assert()
            .success()
            .stdout(
                "confbk-2018_12_15/\n\
                 confbk-2018_12_15/backMeUp1\n\
                 confbk-2018_12_15/backMeUp2\n",
            );
        fs::remove_file(backup_file).expect("Cannot remove file");
    } else {
        assert!(file.exists());
    }
}

#[test]
fn list_and_file() {
    confbk("list_and_file_env")
        .arg("-f")
        .arg("listOfConfigs1-2")
        .arg("-l")
        .arg("backMeUp3")
        .assert()
        .success()
        .stdout("Backing up\n");
    let time = Local::now().format("%Y_%m_%d");
    let backup_dir = format!("{}list_and_file_env/confbk-{}", CURRENT_DIR, time);
    let dir = PathBuf::from(&backup_dir);
    if dir.exists() {
        let dir = fs::read_dir(dir).expect("Failed to open directory");
        let files: Vec<OsString> = dir
            .map(|f| f.unwrap().path().file_stem().unwrap().to_os_string())
            .collect();

        let file1: &OsString = files.get(0).unwrap();
        let file2: &OsString = files.get(1).unwrap();
        let file3: &OsString = files.get(2).unwrap();
        assert_eq!(file1, "backMeUp1");
        assert_eq!(file2, "backMeUp2");
        assert_eq!(file3, "backMeUp3");
        fs::remove_dir_all(backup_dir).expect("Couldn't remove directory");
    } else {
        assert!(dir.exists());
    }
}

#[test]
fn dry_run() {
    confbk("file_env")
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
    let time = Local::now().format("%Y_%m_%d");
    let stdout = format!(
        "[Debug] Params {{\n    \
         out: \"confbk-{}\",\n    \
         dry_run: true,\n    \
         quiet: false,\n    \
         verbose: true,\n    \
         file_of_configs: None,\n    \
         list_of_configs: Some(\n        \
         [\n            \
         \"backMeUp1\"\n        \
         ]\n    \
         ),\n    \
         tar: false\n\
         }}\n\
         Files to be backed up:\n    \
         backMeUp1\n",
        time
    );
    confbk("file_env")
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
    confbk("file_env")
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
    confbk("file_env")
        .arg("-l")
        .arg("backMeUp1")
        .arg("-v")
        .arg("-q")
        .assert()
        .failure();
}

#[test]
fn file_in_list_doesnt_exist() {
    confbk("file_env")
        .arg("-l")
        .arg("IDontExist")
        .assert()
        .failure();
}

#[test]
fn file_in_file_doesnt_exist() {
    confbk("file_in_file_doesnt_exist_env")
        .arg("-f")
        .arg("listOfNonExistentConfig")
        .assert()
        .failure();
}