
pub struct Flags {
    nocolor: bool,
}

impl Flags {
    pub fn new(nocolor: bool) -> Self {
        Self {
            nocolor,
        }
    }

    pub fn is_nocolor(&self) -> bool {
        self.nocolor
    }
}
