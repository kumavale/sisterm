use std::sync::Mutex;

use lazy_static::lazy_static;


#[allow(dead_code)]
mod commands {
    pub const SE   :u8 = 0xF0;  // 240
    pub const NOP  :u8 = 0xF1;  // 241
    pub const DM   :u8 = 0xF2;  // 242
    pub const BRK  :u8 = 0xF3;  // 243
    pub const IP   :u8 = 0xF4;  // 244
    pub const AO   :u8 = 0xF5;  // 245
    pub const AYT  :u8 = 0xF6;  // 246
    pub const EC   :u8 = 0xF7;  // 247
    pub const EL   :u8 = 0xF8;  // 248
    pub const GA   :u8 = 0xF9;  // 249
    pub const SB   :u8 = 0xFA;  // 250

    pub const WILL :u8 = 0xFB;  // 251
    pub const WONT :u8 = 0xFC;  // 252
    pub const DO   :u8 = 0xFD;  // 253
    pub const DONT :u8 = 0xFE;  // 254

    pub const IAC  :u8 = 0xFF;  // 255

    // Authentication Types
    pub const IS    :u8 = 0x00;
    pub const SEND  :u8 = 0x01;
    pub const REPLY :u8 = 0x02;
    pub const NAME  :u8 = 0x03;

    // NEW-ENVIRON
    pub const VAR     :u8 = 0x00;
    pub const VALUE   :u8 = 0x01;
    pub const ESC     :u8 = 0x02;
    pub const USERVAR :u8 = 0x03;
}

#[allow(dead_code)]
mod options {
    pub const BINARY_TRANSMISSION   :u8 = 0x00;  //  0
    pub const ECHO                  :u8 = 0x01;  //  1
    //pub const                       :u8 = 0x02;  //  2
    pub const SUPPRESS_GO_AHEAD     :u8 = 0x03;  //  3
    //pub const                       :u8 = 0x04;  //  4
    pub const STATUS                :u8 = 0x05;  //  5
    pub const TIMING_MARK           :u8 = 0x06;  //  6
    //pub const                       :u8 = 0x07;  //  7
    //pub const                       :u8 = 0x08;  //  8
    //pub const                       :u8 = 0x09;  //  9
    //pub const                       :u8 = 0x0A;  // 10
    //pub const                       :u8 = 0x0B;  // 11
    //pub const                       :u8 = 0x0C;  // 12
    //pub const                       :u8 = 0x0D;  // 13
    //pub const                       :u8 = 0x0E;  // 14
    //pub const                       :u8 = 0x0F;  // 15
    //pub const                       :u8 = 0x10;  // 16
    //pub const                       :u8 = 0x11;  // 17
    //pub const                       :u8 = 0x12;  // 18
    //pub const                       :u8 = 0x13;  // 19
    //pub const                       :u8 = 0x14;  // 20
    //pub const                       :u8 = 0x15;  // 21
    //pub const                       :u8 = 0x16;  // 22
    //pub const                       :u8 = 0x17;  // 23
    pub const TERMINAL_TYPE         :u8 = 0x18;  // 24
    //pub const                       :u8 = 0x19;  // 25
    //pub const                       :u8 = 0x1A;  // 26
    //pub const                       :u8 = 0x1B;  // 27
    //pub const                       :u8 = 0x1C;  // 28
    //pub const                       :u8 = 0x1D;  // 29
    //pub const                       :u8 = 0x1E;  // 30
    pub const WINDOW_SIZE           :u8 = 0x1F;  // 31
    pub const TERMINAL_SPEED        :u8 = 0x20;  // 32
    pub const REMOTE_FLOW_CONTROL   :u8 = 0x21;  // 33
    pub const LINE_MODE             :u8 = 0x22;  // 34
    pub const X_DISPLAY_LOCATION    :u8 = 0x23;  // 35
    pub const ENVIRONMENT_VARIABLES :u8 = 0x24;  // 36
    //pub const                       :u8 = 0x25;  // 37
    //pub const                       :u8 = 0x26;  // 38
    pub const NEW_ENVIRONMENT       :u8 = 0x27;  // 39
    //pub const                       :u8 = 0x28;  // 40
    //pub const                       :u8 = 0x29;  // 41
    //pub const                       :u8 = 0x2A;  // 42
    //pub const                       :u8 = 0x2B;  // 43
    //pub const                       :u8 = 0x2C;  // 44
    //pub const                       :u8 = 0x2D;  // 45
    //pub const                       :u8 = 0x2E;  // 46
    //pub const                       :u8 = 0x2F;  // 47
    //pub const                       :u8 = 0x30;  // 48
    //pub const                       :u8 = 0x31;  // 49
    /* ------------   50-137 Unassigned  ------------ */
    //pub const                       :u8 = 0x8A;  // 138
    //pub const                       :u8 = 0x8B;  // 139
    //pub const                       :u8 = 0x8C;  // 140
    /* ------------  141-254 Unassigned  ------------ */
    //pub const                       :u8 = 0xFF;  // 255
}

lazy_static! { static ref LOGIN_USER:    Mutex<String> = Mutex::new(String::new()); }
lazy_static! { static ref TERMINAL_TYPE: Mutex<String> = Mutex::new(String::new()); }

pub fn init(transmitter: &mut std::net::TcpStream, login_user: Option<&str>, terminal_type: &str) {
    use std::io::Write;

    // Initiarize default login user
    if let Some(username) = login_user {
        *LOGIN_USER.lock().unwrap() = username.to_string();
    }
    *TERMINAL_TYPE.lock().unwrap() = terminal_type.to_string();

    // Negotiations sent first
    let data = [
        commands::IAC, commands::DO,   options::SUPPRESS_GO_AHEAD,
        commands::IAC, commands::WILL, options::TERMINAL_TYPE,
        commands::IAC, commands::WILL, options::WINDOW_SIZE,
        commands::IAC, commands::WILL, options::TERMINAL_SPEED,
        commands::IAC, commands::WILL, options::REMOTE_FLOW_CONTROL,
        //commands::IAC, commands::WILL, options::LINE_MODE,
        commands::IAC, commands::WILL, options::NEW_ENVIRONMENT,
        commands::IAC, commands::DO,   options::STATUS,
    ];

    if let Err(e) = transmitter.write(&data) {
        eprintln!("{}", e);
    }
}

pub fn parse_commands(t: usize, serial_buf: &[u8], send_neg: &mut Vec<u8>) -> String {
    let mut output = String::new();
    let mut i = 0;

    while i < t {
        if serial_buf[i] != commands::IAC {
            if let Some(ch) = is_char(serial_buf, &mut i) {
                output.push(ch);
            }
            i += 1;
            continue;
        }

        i += 1;

        match serial_buf[i] {
            commands::WILL => {
                i += 1;
                match serial_buf[i] {
                    options::ECHO =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::DO,
                            options::ECHO,
                        ]),
                    options::SUPPRESS_GO_AHEAD =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::DO,
                            options::SUPPRESS_GO_AHEAD,
                        ]),
                    options::TERMINAL_TYPE =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::DO,
                            options::TERMINAL_TYPE,
                        ]),
                    option =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::DONT,
                            option,
                        ]),
                }
            },
            commands::WONT => {
                i += 1;
                send_neg.extend_from_slice(&[
                    commands::IAC,
                    commands::WONT,
                    serial_buf[i],
                ]);
            },
            commands::DO   => {
                i += 1;
                match serial_buf[i] {
                    options::SUPPRESS_GO_AHEAD =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WILL,
                            options::SUPPRESS_GO_AHEAD,
                        ]),
                    options::WINDOW_SIZE => {
                        send_neg.extend_from_slice(&window_size());
                    },
                    options::TERMINAL_TYPE =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WILL,
                            options::TERMINAL_TYPE,
                        ]),
                    options::TERMINAL_SPEED =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WILL,
                            options::TERMINAL_SPEED,
                        ]),
                    options::ECHO =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WONT,
                            options::ECHO,
                        ]),
                    options::LINE_MODE =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WONT,
                            options::LINE_MODE,
                        ]),
                    options::REMOTE_FLOW_CONTROL =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WONT,
                            options::ECHO,
                        ]),
                    options::NEW_ENVIRONMENT =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            if (*LOGIN_USER.lock().unwrap()).is_empty() {
                                commands::WONT
                            } else {
                                commands::WILL
                            },
                            options::NEW_ENVIRONMENT,
                        ]),
                    option => send_neg.extend_from_slice(&[
                        commands::IAC,
                        commands::WONT,
                        option,
                    ]),
                }
            },
            commands::DONT => {
                i += 1;
                send_neg.extend_from_slice(&[
                    commands::IAC,
                    commands::DONT,
                    serial_buf[i],
                ]);
            },
            commands::SB => {
                i += 1;
                match serial_buf[i] {
                    options::TERMINAL_TYPE => {
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::SB,
                            options::TERMINAL_TYPE,
                            commands::IS,
                        ]);
                        send_neg.extend_from_slice(&
                            //"XTERM".as_bytes(),
                            //"XTERM-256COLOR".as_bytes(),
                            //"VT100".as_bytes(),
                            //"VT200".as_bytes(),
                            //"ANSI".as_bytes(),
                            TERMINAL_TYPE.lock().unwrap().as_bytes(),
                        );
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::SE,
                        ]);
                    },
                    options::TERMINAL_SPEED =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::SB,
                            options::TERMINAL_SPEED,
                            commands::IS,
                            0x33, 0x38, 0x34, 0x30, 0x30,  // 38400
                            0x2C,                          // ,
                            0x33, 0x38, 0x34, 0x30, 0x30,  // 38400
                            commands::IAC,
                            commands::SE,
                        ]),
                    options::NEW_ENVIRONMENT => {
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::SB,
                            options::NEW_ENVIRONMENT,
                            commands::IS,
                            commands::VAR,
                        ]);
                        send_neg.extend_from_slice(&[
                            b'U',b'S',b'E',b'R', // USER
                            commands::VALUE,
                        ]);
                        send_neg.extend_from_slice(&*LOGIN_USER.lock().unwrap().as_bytes());
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::SE,
                        ]);
                    },
                    _ => (),
                }
                while serial_buf[i] != commands::SE { i += 1; }
            },
            _ => (),
        }

        i += 1;
    }

    output
}

pub fn window_size() -> [u8; 9] {
    let mut ret = [0; 9];
    let mut neg = Vec::new();
    neg.extend_from_slice(&[
        commands::IAC,
        commands::SB,
        options::WINDOW_SIZE,
    ]);
    neg.extend_from_slice(&get_window_size());
    neg.extend_from_slice(&[
        commands::IAC,
        commands::SE,
    ]);

    ret.copy_from_slice(&neg[..9]);
    ret
}

#[cfg(windows)]
pub fn get_window_size() -> [u8; 4] {
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{
        GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
    };

    let zc = COORD { X: 0, Y: 0 };
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize:           zc,
        dwCursorPosition: zc,
        wAttributes:       0,
        srWindow: SMALL_RECT {
            Left:   0,
            Top:    0,
            Right:  0,
            Bottom: 0,
        },
        dwMaximumWindowSize: zc,
    };

    if unsafe { GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut csbi) } == 0 {
        return [0, 0x50, 0, 0x18];  // 80x24
    }

    let width  = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;
    let height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;

    [((width & 0b1111_1111_0000_0000) >> 8) as u8,  (width & 0b0000_0000_1111_1111) as u8,
    ((height & 0b1111_1111_0000_0000) >> 8) as u8, (height & 0b0000_0000_1111_1111) as u8]
}
#[cfg(not(windows))]
pub fn get_window_size() -> [u8; 4] {
    use libc::{ioctl, winsize, TIOCGWINSZ, STDOUT_FILENO};
    use std::mem;

    let fd = STDOUT_FILENO;
    let mut ws: winsize = unsafe { mem::zeroed() };

    if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        return [0, 0x50, 0, 0x18];  // 80x24
    }

    let width:  u16 = ws.ws_col;
    let height: u16 = ws.ws_row;

    [((width & 0b1111_1111_0000_0000) >> 8) as u8,  (width & 0b0000_0000_1111_1111) as u8,
    ((height & 0b1111_1111_0000_0000) >> 8) as u8, (height & 0b0000_0000_1111_1111) as u8]
}

fn is_char(buf: &[u8], i: &mut usize) -> Option<char> {
    let mut d = *i;

    if buf[d].is_ascii() {
        Some(buf[d] as char)
    } else {
        let bytes = &mut Vec::new();
        bytes.push(buf[d]);

        loop {
            d += 1;
            if buf.len() <= d {
                return None;
            }
            bytes.push(buf[d]);
            if let Ok(st) = std::str::from_utf8(bytes) {
                *i = d;
                return Some(st.chars().next().unwrap());
            }
            if 4 <= bytes.len() {
                return None;
            }
        }
    }
}

