extern crate clap;
extern crate rand;

use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;
mod args;
mod util;

fn main() {
    // get arguments passed in
    let arguments = args::parse_args();

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
    // Both files and configs are set
    if arguments.file_of_configs().is_some() && arguments.list_of_configs().is_some() {
        let mut config_list = arguments.file_of_configs().clone().unwrap();
        config_list.append(&mut arguments.list_of_configs().clone().unwrap());
        backup(
            config_list,
            print,
            arguments.out().to_string(),
            arguments.dry_run(),
            arguments.tar(),
        )
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });
    } else if arguments.file_of_configs().is_some() {
        let config_list = arguments.file_of_configs().clone().unwrap();
        backup(
            config_list,
            print,
            arguments.out().to_string(),
            arguments.dry_run(),
            arguments.tar(),
        )
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });
    } else if arguments.list_of_configs().is_some() {
        let config_list = arguments.list_of_configs().clone().unwrap();
        backup(
            config_list,
            print,
            arguments.out().to_string(),
            arguments.dry_run(),
            arguments.tar(),
        )
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        });
    }
}

// Backup function that will backup files
fn backup(
    list: Vec<PathBuf>,
    print: util::VerbosePrint,
    out: String,
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
    let out = PathBuf::from(out);
    fs::create_dir(&out).unwrap_or_else(|_| {
        eprintln!("Error: Directory \"{}\" already exists", out.display());
        process::exit(1);
    });
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
