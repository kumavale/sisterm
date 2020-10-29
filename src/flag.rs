
#[derive(Clone)]
pub struct Flags {
    nocolor:    bool,
    timestamp:  bool,
    append:     bool,
    crlf:       bool,
    debug:      bool,
    write_file: Option<String>,
}

impl Flags {
    pub fn new(nocolor: bool, timestamp: bool, append: bool, crlf: bool, debug: bool,
        write_file: Option<String>) -> Self
    {
        Self {
            nocolor,
            timestamp,
            append,
            crlf,
            debug,
            write_file,
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

    pub fn is_crlf(&self) -> bool {
        self.crlf
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn write_file(&self) -> Option<String> {
        self.write_file.clone()
    }
}
