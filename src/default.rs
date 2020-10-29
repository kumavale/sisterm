pub const READ_BUFFER_SIZE:    usize = 1024;
pub const TCP_CONNECT_TIMEOUT:   u64 = 5;
pub const TIMESTAMP_FORMAT:     &str = "[%Y-%m-%d %H:%M:%S %Z] ";
pub const LOG_FORMAT:           &str = "%Y%m%d_%H%M%S.log";
pub const LOG_DESTINATION:      &str = "./";
pub const TERMINAL_TYPE:        &str = "ANSI";

/// Escape signals
pub mod escape_signals {
    use crate::getch::Key;

    pub const ESCAPE_SIGNAL: Key = Key::Char('~');
    pub const EXIT_CHAR_0:   Key = Key::Char('.');
    pub const EXIT_CHAR_1:   Key = Key::Ctrl('d');
}
