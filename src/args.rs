// Con

use chrono::Local;
use clap::{crate_authors, crate_version, App, Arg};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process;

// All available parameters
#[derive(Debug)]
pub struct Params {
    out: String,
    dry_run: bool,
    quiet: bool,
    verbose: bool,
    file_of_configs: Option<Vec<PathBuf>>,
    list_of_configs: Option<Vec<PathBuf>>,
    tar: bool,
}

impl Params {
    pub fn out(&self) -> &String {
        &self.out
    }
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
    pub fn quiet(&self) -> bool {
        self.quiet
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
    pub fn file_of_configs(&self) -> &Option<Vec<PathBuf>> {
        &self.file_of_configs
    }
    pub fn list_of_configs(&self) -> &Option<Vec<PathBuf>> {
        &self.list_of_configs
    }
    pub fn tar(&self) -> bool {
        self.tar
    }
}

// Parses command line arguments and returns a struct with them all in it
pub fn parse_args() -> Params {
    // Parses arguments
    let matches = App::new("ConfBK")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Backup important configuration files")
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .takes_value(true)
                .value_name("DIR")
                .help("Directory to put the configs in"),
        )
        .arg(
            Arg::with_name("dry-run")
                .short("d")
                .long("dry-run")
                .help("List files that would be backed up"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .conflicts_with("verbose")
                .help("Do not display any output"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .conflicts_with("quiet")
                .help("Display more verbose output"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("FILE")
                .required_unless("list")
                .help("A file that contains filenames of configs (new line delimited)"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .takes_value(true)
                .value_name("FILE")
                .multiple(true)
                .required_unless("file")
                .help("A list of config files to be backed up"),
        )
        .arg(
            Arg::with_name("tar-xz")
                .short("x")
                .long("tar-xz")
                .help("Compress config dir into a .tar.xz file"),
        )
        .get_matches();

    let time = Local::now().format("%Y_%m_%d");

    // If out isn't explicitly set, it will be set to confbk- followed by the current date
    let out = matches
        .value_of("out")
        .unwrap_or(&format!("confbk-{}", time))
        .to_string();

    let dry_run = matches.is_present("dry-run");

    let quiet = matches.is_present("quiet");

    let verbose = matches.is_present("verbose");

    // Get the Option<&str> of the file_of_configs file
    let file_of_configs = matches.value_of("file");

    // Turn the string into an owned and valid Path
    let file_of_configs: Option<Vec<PathBuf>> = match file_of_configs {
        Some(file_name) => {
            let path = PathBuf::from(file_name);
            if path.exists() {
                let configs = fs::File::open(path).unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                });
                let buf = BufReader::new(configs);
                let configs: Vec<String> = buf
                    .lines()
                    .map(|l| {
                        l.unwrap_or_else(|e| {
                            eprintln!("Error: {}", e);
                            process::exit(1)
                        })
                    })
                    .collect();
                let paths: Vec<PathBuf> = configs.iter().map(PathBuf::from).collect();
                let mut valid_paths = Vec::new();
                for path in paths {
                    if path.exists() {
                        valid_paths.push(path);
                    } else {
                        eprintln!("Error: {}: No such file", path.display());
                        process::exit(1);
                    }
                }
                Some(valid_paths)
            } else {
                eprintln!("Errror: {}: No such file", file_name);
                process::exit(1)
            }
        }
        None => None,
    };

    // Get the vector of Option<&str> to be converted
    let list_of_configs: Option<Vec<String>> = match matches.values_of("list") {
        Some(cfgs) => Some(cfgs.map(str::to_string).collect()),
        None => None,
    };
    let list_of_configs: Option<Vec<PathBuf>> = match list_of_configs {
        Some(file_names) => {
            let paths = file_names.iter().map(PathBuf::from);
            let mut valid_paths = Vec::new();
            for path in paths {
                if path.exists() {
                    valid_paths.push(path);
                } else {
                    eprintln!("Error: {}: No such file", path.display());
                    process::exit(1);
                }
            }
            Some(valid_paths)
        }
        None => None,
    };

    let tar = matches.is_present("tar-xz");

    Params {
        out,
        dry_run,
        quiet,
        verbose,
        list_of_configs,
        file_of_configs,
        tar,
    }
}
