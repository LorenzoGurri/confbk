use super::util;
use duct::cmd;
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
    files: &[PathBuf],
    directories: &[PathBuf],
    print: &util::VerbosePrint,
    out: &PathBuf,
    dry_run: bool,
    tar: bool,
) -> io::Result<()> {
    if dry_run {
        print.println("Files to be backed up:");
        for file in files {
            print.println(&format!("    {}", file.display()));
        }
        return Ok(());
    }
    print.println("Backing up");
    fs::create_dir(&out)?;
    for file in files {
        print.debug(&format!(
            "Copying file \"{}\" to \"{}\"",
            file.display(),
            out.display()
        ));
        cmd!("cp", file, out).stdout_null().run().unwrap();
    }
    for directory in directories {
        print.debug(&format!(
            "Copying directory \"{}\" to \"{}\"",
            directory.display(),
            out.display()
        ));
        cmd!("cp", "-r", directory, out)
            .stdout_null()
            .run()
            .unwrap();
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
