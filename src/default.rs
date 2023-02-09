pub const READ_BUFFER_SIZE:    usize = 1024;
pub const TCP_CONNECT_TIMEOUT:   u64 = 5;
pub const RETRY_AUTH_COUNT:    usize = 3;
pub const TIMESTAMP_FORMAT:     &str = "[%Y-%m-%d %H:%M:%S %Z] ";
pub const LOG_FORMAT:           &str = "%Y%m%d_%H%M%S.log";
pub const LOG_DESTINATION:      &str = "./";
pub const TERMINAL_TYPE:        &str = "xterm";

/// Escape sequences
pub mod escape_sequences {
    use getch_rs::Key;

    pub const ESCAPE_SIGNAL_0: Key = Key::Char('~');
    pub const ESCAPE_SIGNAL_1: Key = Key::Alt('~');
    pub const EXIT_CHAR_0:     Key = Key::Char('.');
    pub const EXIT_CHAR_1:     Key = Key::Ctrl('d');
    pub const SUSPEND:         Key = Key::Ctrl('z');
    pub const NO_COLOR:        Key = Key::Char('n');
    pub const TIME_STAMP:      Key = Key::Char('t');
    pub const INSTEAD_CRLF:    Key = Key::Char('i');
    pub const HEXDUMP:         Key = Key::Char('h');
    pub const DEBUG:           Key = Key::Char('d');
    pub const COMMAND_0:       Key = Key::Char('!');
    pub const COMMAND_1:       Key = Key::Char('$');
    pub const HELP:            Key = Key::Char('?');
}

pub fn terminal_type() -> String {
    if let Ok(t) = std::env::var("TERM") {
        t
    } else {
        TERMINAL_TYPE.to_string()
    }
}
