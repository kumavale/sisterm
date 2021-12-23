
#[derive(Clone)]
pub struct Flags {
    nocolor:      bool,
    timestamp:    bool,
    append:       bool,
    instead_crlf: bool,
    hexdump:      bool,
    debug:        bool,
    write_file:   Option<String>,
}

impl Flags {
    pub fn new(nocolor: bool, timestamp: bool, append: bool, instead_crlf: bool, debug: bool,
        write_file: Option<String>) -> Self
    {
        Self {
            nocolor,
            timestamp,
            append,
            instead_crlf,
            hexdump: false,
            debug,
            write_file,
        }
    }

    pub fn nocolor(&self)              -> &bool     { &self.nocolor }
    pub fn nocolor_mut(&mut self)      -> &mut bool { &mut self.nocolor }
    pub fn timestamp(&self)            -> &bool     { &self.timestamp }
    pub fn timestamp_mut(&mut self)    -> &mut bool { &mut self.timestamp }
    pub fn append(&self)               -> &bool     { &self.append }
    pub fn append_mut(&mut self)       -> &mut bool { &mut self.append }
    pub fn instead_crlf(&self)         -> &bool     { &self.instead_crlf }
    pub fn instead_crlf_mut(&mut self) -> &mut bool { &mut self.instead_crlf }
    pub fn hexdump(&self)              -> &bool     { &self.hexdump }
    pub fn hexdump_mut(&mut self)      -> &mut bool { &mut self.hexdump }
    pub fn debug(&self)                -> &bool     { &self.debug }
    pub fn debug_mut(&mut self)        -> &mut bool { &mut self.debug }

    pub fn write_file(&self) -> Option<String> {
        self.write_file.clone()
    }
}
