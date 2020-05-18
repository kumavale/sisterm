
pub struct Flags {
    nocolor:     bool,
    timestamp:   bool,
    write_file:  Option<String>,
    config_file: String,
}

impl Flags {
    pub fn new(nocolor: bool, timestamp: bool, wf: Option<&str>, cf: &str) -> Self {
        Self {
            nocolor,
            timestamp,
            write_file: match wf {
                Some(file) => Some(file.to_string()),
                None => None,
            },
            config_file: cf.to_string(),
        }
    }

    pub fn is_nocolor(&self) -> bool {
        self.nocolor
    }

    pub fn is_timestamp(&self) -> bool {
        self.timestamp
    }

    pub fn write_file(&self) -> Option<&String> {
        self.write_file.as_ref()
    }
}
