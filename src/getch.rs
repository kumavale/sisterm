
#[cfg(windows)]
use libc::c_int;
#[cfg(windows)]
extern "C" {
    fn _getch() -> c_int;
}

#[cfg(not(windows))]
use nix::sys::termios;
#[cfg(not(windows))]
use std::io::Read;


#[cfg(windows)]
pub struct Getch {}

#[cfg(not(windows))]
pub struct Getch {
    orig_term: termios::Termios,
}


/// A key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// Null byte.
    Null,
    /// Backspace.
    Backspace,
    /// Delete key.
    Delete,
    /// Esc key.
    Esc,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Right arrow.
    Right,
    /// Left arrow.
    Left,
    /// End key.
    End,
    /// Home key.
    Home,
    /// Backward Tab key.
    BackTab,
    /// Insert key.
    Insert,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Other key.
    Other(Vec<u8>),
}

impl Getch {
    #[cfg(windows)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
    #[cfg(not(windows))]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Quering original as a separate, since `Termios` does not implement copy
        let orig_term       = termios::tcgetattr(0).unwrap();
        let mut raw_termios = termios::tcgetattr(0).unwrap();

        // Unset canonical mode, so we get characters immediately
        raw_termios.local_flags.remove(termios::LocalFlags::ICANON);
        // Don't generate signals on Ctrl-C and friends
        raw_termios.local_flags.remove(termios::LocalFlags::ISIG);
        // Disable local echo
        raw_termios.local_flags.remove(termios::LocalFlags::ECHO);

        termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &raw_termios).unwrap();

        Self {
            orig_term
        }
    }

    #[cfg(windows)]
    pub fn getch(&self) -> Result<Key, std::io::Error> {
        todo!();
        loop {
            unsafe {
                let key = _getch();
                if key == 0 {
                    // Ignore next input
                    _getch();
                } else {
                    match key {
                        0x8 => Ok(Key::Backspace),
                        c   => Ok(Key::Char(c as char)),
                    }
                }
            }
        }
    }
    #[cfg(not(windows))]
    pub fn getch(&self) -> Result<Key, std::io::Error> {
        let mut r: [u8; 1] = [0];

        std::io::stdin().read_exact(&mut r[..])?;
        match r[0] {
            b'\x1B' => {
                std::io::stdin().read_exact(&mut r[..])?;
                Ok(match r[0] {
                    b'[' => parse_csi()?,
                    b'O' => {
                        std::io::stdin().read_exact(&mut r[..])?;
                        match r[0] {
                            // F1-F4
                            val @ b'P'..=b'S' => Key::F(1 + val - b'P'),
                            _ => Key::Other(vec![ b'\x1B', b'O', r[0] ]),
                        }
                    },
                    c => {
                        match parse_utf8_char(c)? {
                            Ok(ch) =>   Key::Alt(ch),
                            Err(vec) => Key::Other(vec),
                        }
                    },
                })
            },
            b'\n' | b'\r'         => Ok(Key::Char('\r')),
            b'\t'                 => Ok(Key::Char('\t')),
            b'\x08'               => Ok(Key::Backspace),
            b'\x7F'               => Ok(Key::Delete),
            c @ b'\x01'..=b'\x1A' => Ok(Key::Ctrl((c as u8 - 0x1  + b'a') as char)),
            c @ b'\x1C'..=b'\x1F' => Ok(Key::Ctrl((c as u8 - 0x1C + b'4') as char)),
            b'\0'                 => Ok(Key::Null),
            c => {
                Ok(
                    match parse_utf8_char(c)? {
                        Ok(ch) => Key::Char(ch),
                        Err(vec) => Key::Other(vec),
                    }
                )
            }
        }
    }
}

/// Parses a CSI sequence, just after reading ^[
///
/// Returns None if an unrecognized sequence is found.
fn parse_csi() -> Result<Key, std::io::Error> {
    let mut r: [u8; 1] = [0];

    std::io::stdin().read_exact(&mut r[..])?;
    Ok(match r[0] {
        b'[' => {
            std::io::stdin().read_exact(&mut r[..])?;
            match r[0] {
                val @ b'A'..=b'E' => Key::F(1 + val - b'A'),
                _ => Key::Other(vec![ b'\x1B', b'[', b'[', r[0] ]),
            }
        },
        b'A' => Key::Up,
        b'B' => Key::Down,
        b'C' => Key::Right,
        b'D' => Key::Left,
        b'F' => Key::End,
        b'H' => Key::Home,
        b'Z' => Key::BackTab,
        c @ b'0'..=b'9' => {
            // Numbered escape code.
            let mut buf = Vec::new();
            buf.push(c);
            std::io::stdin().read_exact(&mut r[..])?;
            let mut c = r[0];
            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while c < 64 || c > 126 {
                buf.push(c);
                std::io::stdin().read_exact(&mut r[..])?;
                c = r[0];
            }
            match c {
                // Special key code.
                b'~' => {
                    let str_buf = std::str::from_utf8(&buf).unwrap();

                    // This CSI sequence can be a list of semicolon-separated
                    // numbers.
                    let nums: Vec<u8> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    if nums.is_empty() || nums.len() > 1 {
                        let mut keys = vec![ b'\x1B', b'['];
                        keys.append(&mut buf);
                        return Ok(Key::Other(keys));
                    }

                    match nums[0] {
                        1 | 7 => Key::Home,
                        2     => Key::Insert,
                        3     => Key::Delete,
                        4 | 8 => Key::End,
                        5     => Key::PageUp,
                        6     => Key::PageDown,
                        v @ 11..=15 => Key::F(v - 10),
                        v @ 17..=21 => Key::F(v - 11),
                        v @ 23..=24 => Key::F(v - 12),
                        _ => {
                            let mut keys = vec![ b'\x1B', b'['];
                            keys.append(&mut buf);
                            return Ok(Key::Other(keys));
                        },
                    }
                },
                _ => {
                    let mut keys = vec![ b'\x1B', b'['];
                    keys.append(&mut buf);
                    return Ok(Key::Other(keys));
                },
            }
        },
        _ => Key::Other(vec![ b'\x1B', b'[', r[0] ]),
    })
}

/// Parse `c` as either a single byte ASCII char or a variable size UTF-8 char.
fn parse_utf8_char(c: u8) -> Result<Result<char, Vec<u8>>, std::io::Error> {
    if c.is_ascii() {
        Ok(Ok(c as char))
    } else {
        let mut r: [u8; 1] = [0];
        let bytes = &mut Vec::new();
        bytes.push(c);

        loop {
            std::io::stdin().read_exact(&mut r[..])?;
            bytes.push(r[0]);
            if let Ok(st) = std::str::from_utf8(bytes) {
                return Ok(Ok(st.chars().next().unwrap()));
            }
            if bytes.len() >= 4 {
                return Ok(Err(bytes.to_vec()));
            }
        }
    }
}

impl Drop for Getch {
    #[cfg(windows)]
    fn drop(&mut self) {}

    #[cfg(not(windows))]
    fn drop(&mut self) {
        termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &self.orig_term).unwrap();
    }

}
