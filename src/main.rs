extern crate duct;
extern crate fs_extra;
extern crate structopt;

mod args;
mod util;

use std::path::PathBuf;

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

    let paths = arguments.validate_paths().unwrap();
    print.debug(&format!("{:#?}", arguments));
    let path = PathBuf::from("confbk_backup");
    let out_file = match arguments.out() {
        Some(s) => s,
        None => &path,
    };
    util::backup(
        &paths,
        &print,
        &out_file,
        arguments.dry_run(),
        arguments.tar(),
    )
    .unwrap_or_else(|e| util::FatalError::error(&e.to_string()));
}
