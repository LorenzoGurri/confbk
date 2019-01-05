use super::util;
use duct::cmd;
use fs_extra::dir::{self, DirContent};
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;
pub enum VerboseLevel {
    On,
    Reg,
    Off,
}

pub struct FatalError();

impl FatalError {
    pub fn file_not_found(file_name: &str) {
        eprintln!("Error: File {} not found", file_name);
        process::exit(1);
    }
    pub fn error(msg: &str) {
        eprintln!("{}", msg);
        process::exit(1);
    }
}

pub struct VerbosePrint {
    pub level: VerboseLevel,
}

impl VerbosePrint {
    pub fn debug(&self, s: &str) {
        if let VerboseLevel::On = self.level {
            println!("[Debug] {}", s);
        }
    }
    pub fn println(&self, s: &str) {
        match self.level {
            VerboseLevel::On | VerboseLevel::Reg => println!("{}", s),
            VerboseLevel::Off => (),
        }
    }
}

// Backup function that will backup files
pub fn backup(
    paths: &[PathBuf],
    print: &util::VerbosePrint,
    out: &std::path::Path,
    dry_run: bool,
    tar: bool,
) -> io::Result<()> {
    if dry_run {
        print.println("Files to be backed up:");
        for file in paths {
            print.println(&format!("    {}", file.display()));
        }
        return Ok(());
    }
    print.println("Backing up");
    fs::create_dir(&out)?;
    for file in paths {
        print.debug(&format!(
            "Copying file \"{}\" to \"{}\"",
            file.display(),
            out.display()
        ));
        let parts: Vec<&OsStr> = file.iter().collect();
        let mut path = OsString::new();
        if parts.len() > 1 {
            let dir_path: Vec<PathBuf> = parts
                .iter()
                .take(parts.len() - 1)
                .map(OsString::from)
                .map(PathBuf::from)
                .collect();
            for dir in dir_path {
                path.push(dir);
            }
            out.to_path_buf().push(&path);
            dir::create_all(PathBuf::from(&out), false).expect("Failed to create directories");
        }

        let mut out = PathBuf::from(out);
        out.push(path);
        cmd!("mkdir", "-p", &out).stdout_null().run().unwrap();
        cmd!("cp", "-r", file, &out).stdout_null().run().unwrap();
    }
    if tar {
        print.debug("Executing Tar");
        cmd(
            "tar",
            &[
                "cjf",
                &format!("{}.tar.xz", out.display()),
                &out.display().to_string(),
            ],
        )
        .stdout_null()
        .run()
        .unwrap();
        fs::remove_dir_all(&out)?;
    }

    Ok(())
}

pub fn all_paths(dir: DirContent) -> Vec<PathBuf> {
    let paths = dir.files.iter().map(|f| PathBuf::from(f)).collect();
    paths
}
