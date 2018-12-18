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
