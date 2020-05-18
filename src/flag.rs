
pub struct Flags {
    nocolor:    bool,
    write_file: Option<String>,
}

impl Flags {
    pub fn new(nocolor: bool, wf: Option<&str>) -> Self {
        Self {
            nocolor,
            write_file: match wf {
                Some(file) => Some(file.to_string()),
                None => None,
            },
        }
    }

    pub fn is_nocolor(&self) -> bool {
        self.nocolor
    }

    pub fn write_file(&self) -> Option<&String> {
        self.write_file.as_ref()
    }
}
