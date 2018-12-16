pub enum VerboseLevel {
    On,
    Reg,
    Off,
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
