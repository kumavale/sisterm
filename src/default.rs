pub const READ_BUFFER_SIZE:    usize = 1024;
pub const TCP_CONNECT_TIMEOUT:   u64 = 5;
pub const TIMESTAMP_FORMAT:     &str = "[%Y-%m-%d %H:%M:%S %Z] ";
pub const LOG_FORMAT:           &str = "%Y%m%d_%H%M%S.log";
pub const LOG_DESTINATION:      &str = "./";
pub const TERMINAL_TYPE:        &str = "ANSI";

/// Escape sequences
pub mod escape_sequences {
    use crate::getch::Key;

    pub const ESCAPE_SIGNAL: Key = Key::Char('~');
    pub const EXIT_CHAR_0:   Key = Key::Char('.');
    pub const EXIT_CHAR_1:   Key = Key::Ctrl('d');
    pub const SUSPEND:       Key = Key::Ctrl('z');
    pub const NO_COLOR:      Key = Key::Char('n');
    pub const TIME_STAMP:    Key = Key::Char('t');
    pub const INSTEAD_CRLF:  Key = Key::Char('i');
    pub const DEBUG:         Key = Key::Char('d');
    pub const HELP:          Key = Key::Char('?');
}
