extern crate lazy_static;
extern crate rand;
extern crate structopt;

mod args;
mod util;

//use std::io::{BufRead, BufReader};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

fn main() {
    // get arguments passed in
    let arguments = args::Opt::new();
    // Set verbosity
    let print = if arguments.quiet() {
        util::VerbosePrint {
            level: util::VerboseLevel::Off,
        }
    } else if arguments.verbose() {
        util::VerbosePrint {
            level: util::VerboseLevel::On,
        }
    } else {
        util::VerbosePrint {
            level: util::VerboseLevel::Reg,
        }
    };

    print.debug(&format!("{:#?}", arguments));

    let configs = arguments.validate_files().unwrap();
    let path = PathBuf::from("confbk_backup");
    let out_file = match arguments.out() {
        Some(s) => s,
        None => &path,
    };
    backup(
        &configs,
        &print,
        out_file,
        arguments.dry_run(),
        arguments.tar(),
    )
    .unwrap_or_else(|e| util::FatalError::error(&e.to_string()));
}

// Backup function that will backup files
fn backup(
    list: &[PathBuf],
    print: &util::VerbosePrint,
    out: &PathBuf,
    dry_run: bool,
    tar: bool,
) -> io::Result<()> {
    if dry_run {
        print.println("Files to be backed up:");
        for file in list {
            print.println(&format!("    {}", file.display()));
        }
        return Ok(());
    }
    print.println("Backing up");
    fs::create_dir(&out)?;
    for file in list {
        print.debug(&format!(
            "Copying file \"{}\" to \"{}\"",
            file.display(),
            out.display()
        ));
        process::Command::new("cp")
            .args(&[&file, &out])
            .status()
            .unwrap();
    }
    if tar {
        print.debug("Executing Tar");
        process::Command::new("tar")
            .args(&[
                "cJf",
                &format!("{}.tar.xz", out.display()),
                &out.display().to_string(),
            ])
            .status()
            .unwrap();
        fs::remove_dir_all(&out)?;
    }

    Ok(())
}
