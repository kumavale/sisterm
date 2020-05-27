
#[derive(Clone)]
pub struct Flags {
    nocolor:    bool,
    timestamp:  bool,
    append:     bool,
    instead_cr: bool,
    write_file: Option<String>,
}

impl Flags {
    pub fn new(nocolor: bool, timestamp: bool, append: bool, instead_cr: bool, wf: Option<&str>) -> Self
    {
        Self {
            nocolor,
            timestamp,
            append,
            instead_cr,
            write_file: match wf {
                Some(file) => Some(file.to_string()),
                None => None,
            },
        }
    }

    pub fn is_nocolor(&self) -> bool {
        self.nocolor
    }

    pub fn set_nocolor(&mut self, nocolor: bool) {
        self.nocolor = nocolor;
    }

    pub fn is_timestamp(&self) -> bool {
        self.timestamp
    }

    pub fn is_append(&self) -> bool {
        self.append
    }

    pub fn set_append(&mut self, append: bool) {
        self.append = append;
    }

    pub fn is_instead_cr(&self) -> bool {
        self.instead_cr
    }

    pub fn write_file(&self) -> Option<&String> {
        self.write_file.as_ref()
    }
}
