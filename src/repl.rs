use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::flag;
use crate::color;
use crate::setting;
use crate::getch::{Getch, Key};
use crate::default;
use crate::negotiation;
use crate::hexdump::hexdump;

use chrono::Local;

pub trait Send {
    fn send(&mut self, s: &[u8]);
}
impl<T> Send for T where T: std::io::Write {
    fn send(&mut self, s: &[u8]) {
        if let Err(e) = self.write(s) {
            eprintln!("{}", e);
        }
    }
}

pub async fn receiver<T>(
    mut port: T,
    rx:       std::sync::mpsc::Receiver<()>,
    flags:    Arc<Mutex<flag::Flags>>,
    params:   Option<setting::Params>)
where
    T: std::io::Read,
{
    let (read_buf_size, timestamp_format) = if let Some(ref p) = params {
        (
            p.read_buf_size,
            &*p.timestamp_format
        )
    } else {
        (
            default::READ_BUFFER_SIZE,
            default::TIMESTAMP_FORMAT
        )
    };
    let mut serial_buf: Vec<u8> = vec![0; read_buf_size];
    let mut last_word = (String::new(), false, false);  // (increasing_str, prev_matched, comment_now)

    // Save log
    let write_file = flags.lock().unwrap().write_file();
    if let Some(write_file) = write_file {
        let is_new = !Path::new(&write_file).exists();

        let mut log_file = {
            if *flags.lock().unwrap().append() {
                BufWriter::new(OpenOptions::new()
                    .append(true)
                    .open(&write_file)
                    .expect("File open failed"))
            } else {
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&write_file)
                    .expect("File open failed"))
            }
        };
        let mut log_buf    = String::new();
        let mut write_flag = false;

        println!("Log record: \"{}\" ({})\n", write_file,
            if is_new {
                "New"
            } else if *flags.lock().unwrap().append() {
                "Append"
            } else {
                "Overwrite"
            }
        );

        // First, insert return to log
        log_file.write_all(b"\n").unwrap();

        loop {
            // if "~." is typed, exit
            if rx.try_recv().is_ok() {
                // Write to log
                if *flags.lock().unwrap().timestamp() {
                    let mut log_buf_vec: Vec<&str> = log_buf.split('\n').collect();
                    let log_buf_last = log_buf_vec.pop().unwrap().to_string();
                    log_file.write_all(
                        log_buf_vec
                        .iter()
                        .map(|line| format_timestamp(timestamp_format) + line + "\n")
                        .collect::<String>()
                        .as_bytes()
                    ).unwrap();
                    log_file.write_all(
                        (format_timestamp(timestamp_format) + &log_buf_last)
                        .as_bytes()).unwrap();
                } else {
                    log_file.write_all(log_buf.as_bytes()).unwrap();
                }

                log_file.flush().unwrap();
                break;
            }

            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    if *flags.lock().unwrap().debug() {
                        print!("{:?}", &serial_buf[..t]);
                    }

                    if *flags.lock().unwrap().hexdump() {
                        hexdump(&serial_buf[..t]);
                    } else {
                        // Display after Coloring received string
                        if *flags.lock().unwrap().nocolor() {
                            io::stdout().write_all(&serial_buf[..t]).unwrap();
                        } else {
                            color::coloring_words(
                                &String::from_utf8_lossy(&serial_buf[..t]), &mut last_word, &params);
                        }
                    }

                    // Check exist '\n'
                    for ch in &serial_buf[..t] {
                        // If '\n' exists, set write_flag to true
                        if ch == &b'\n' {
                            write_flag = true;
                            break;
                        }
                    }

                    // Write to log_buf from serial_buf
                    string_from_utf8_appearance(&mut log_buf, &serial_buf[..t]);

                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => eprintln!("{}", e),
            }


            // Flush stdout
            let _ = io::stdout().flush();

            // If end of '\n' then write to log file
            if write_flag {
                // Write timestamp to log file
                if *flags.lock().unwrap().timestamp() {
                    let mut log_buf_vec: Vec<&str> = log_buf.split('\n').collect();
                    let log_buf_last = log_buf_vec.pop().unwrap().to_string();
                    log_file.write_all(
                        log_buf_vec
                            .iter()
                            .map(|line| format_timestamp(timestamp_format) + line + "\n")
                            .collect::<String>()
                            .as_bytes()
                    ).unwrap();
                    log_buf = log_buf_last;
                } else {
                    log_file.write_all(log_buf.as_bytes()).unwrap();
                    log_buf.clear();
                }
                write_flag = false;
            }
        }

    // Non save log
    } else {
        loop {
            // if "~." is typed, exit
            if rx.try_recv().is_ok() {
                break;
            }

            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    if *flags.lock().unwrap().debug() {
                        print!("{:?}", &serial_buf[..t]);
                    }

                    if *flags.lock().unwrap().hexdump() {
                        hexdump(&serial_buf[..t]);
                    } else {
                        // Display after Coloring received string
                        if *flags.lock().unwrap().nocolor() {
                            io::stdout().write_all(&serial_buf[..t]).unwrap();
                        } else {
                            color::coloring_words(
                                &String::from_utf8_lossy(&serial_buf[..t]), &mut last_word, &params);
                        }
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => eprintln!("{}", e),
            }

            // Flush
            let _ = io::stdout().flush();
        }
    }
}

pub async fn receiver_telnet<T>(
    mut port: T,
    rx:       std::sync::mpsc::Receiver<()>,
    flags:    Arc<Mutex<flag::Flags>>,
    params:   Option<setting::Params>)
where
    T: std::io::Read,
    T: std::io::Write,
    T: self::Send,
{
    let (read_buf_size, timestamp_format) = if let Some(ref p) = params {
        (
            p.read_buf_size,
            &*p.timestamp_format
        )
    } else {
        (
            default::READ_BUFFER_SIZE,
            default::TIMESTAMP_FORMAT
        )
    };
    let mut serial_buf: Vec<u8> = vec![0; read_buf_size];
    let mut send_neg:   Vec<u8> = Vec::new();
    let mut last_word = (String::new(), false, false);  // (increasing_str, prev_matched, comment_now)
    let mut window_size = negotiation::get_window_size();

    // Save log
    let write_file = flags.lock().unwrap().write_file();
    if let Some(write_file) = write_file {
        let is_new = !Path::new(&write_file).exists();

        let mut log_file = {
            if *flags.lock().unwrap().append() {
                BufWriter::new(OpenOptions::new()
                    .append(true)
                    .open(&write_file)
                    .expect("File open failed"))
            } else {
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&write_file)
                    .expect("File open failed"))
            }
        };
        let mut log_buf    = String::new();
        let mut write_flag = false;

        println!("Log record: \"{}\" ({})\n", &write_file,
            if is_new {
                "New"
            } else if *flags.lock().unwrap().append() {
                "Append"
            } else {
                "Overwrite"
            }
        );

        // First, insert return to log
        log_file.write_all(b"\n").unwrap();

        loop {
            // if "~." is typed, exit
            if rx.try_recv().is_ok() {
                break;
            }

            // If the window size are different, negotiate
            let window_size_current = negotiation::get_window_size();
            if window_size != window_size_current {
                window_size = window_size_current;
                send_neg.extend_from_slice(&negotiation::window_size());
                port.send(&send_neg[..send_neg.len()]);
                send_neg.clear();
            }

            match port.read(serial_buf.as_mut_slice()) {
                Ok(0) => break,
                Ok(t) => {
                    // Check telnet command
                    let output = negotiation::parse_commands(t, &serial_buf, &mut send_neg);

                    if !send_neg.is_empty() {
                        port.send(&send_neg[..send_neg.len()]);
                        send_neg.clear();
                    }

                    if *flags.lock().unwrap().debug() {
                        print!("{:?}", &serial_buf[..t]);
                    }

                    if *flags.lock().unwrap().hexdump() {
                        hexdump(&serial_buf[..t]);
                    } else {
                        // Display after Coloring received string
                        if *flags.lock().unwrap().nocolor() {
                            io::stdout().write_all(output.as_bytes()).unwrap();
                        } else {
                            color::coloring_words(&output, &mut last_word, &params);
                        }
                    }

                    // Check exist '\n'
                    for ch in output.bytes() {
                        // If '\n' exists, set write_flag to true
                        if ch == b'\n' {
                            write_flag = true;
                            break;
                        }
                    }

                    // Write to log_buf from serial_buf
                    string_from_utf8_appearance(&mut log_buf, output.as_bytes());

                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => eprintln!("{}", e),
            }


            // Flush stdout
            let _ = io::stdout().flush();

            // If end of '\n' then write to log file
            if write_flag {
                // Write timestamp to log file
                if *flags.lock().unwrap().timestamp() {
                    let mut log_buf_vec: Vec<&str> = log_buf.split('\n').collect();
                    let log_buf_last = log_buf_vec.pop().unwrap().to_string();
                    log_file.write_all(
                        log_buf_vec
                            .iter()
                            .map(|line| format_timestamp(timestamp_format) + line + "\n")
                            .collect::<String>()
                            .as_bytes()
                    ).unwrap();
                    log_buf = log_buf_last;
                } else {
                    log_file.write_all(log_buf.as_bytes()).unwrap();
                    log_buf.clear();
                }
                write_flag = false;
            }
        }

        // Write to log
        if *flags.lock().unwrap().timestamp() {
            let mut log_buf_vec: Vec<&str> = log_buf.split('\n').collect();
            let log_buf_last = log_buf_vec.pop().unwrap().to_string();
            log_file.write_all(
                log_buf_vec
                .iter()
                .map(|line| format_timestamp(timestamp_format) + line + "\n")
                .collect::<String>()
                .as_bytes()
            ).unwrap();
            log_file.write_all(
                (format_timestamp(timestamp_format) + &log_buf_last)
                .as_bytes()).unwrap();
                } else {
                    log_file.write_all(log_buf.as_bytes()).unwrap();
        }

        log_file.flush().unwrap();

    // Non save log
    } else {
        loop {
            // if "~." is typed, exit
            if rx.try_recv().is_ok() {
                break;
            }

            // If the window size are different, negotiate
            let window_size_current = negotiation::get_window_size();
            if window_size != window_size_current {
                window_size = window_size_current;
                send_neg.extend_from_slice(&negotiation::window_size());
                port.send(&send_neg[..send_neg.len()]);
                send_neg.clear();
            }

            match port.read(serial_buf.as_mut_slice()) {
                Ok(0) => break,
                Ok(t) => {
                    // Check telnet command
                    let output = negotiation::parse_commands(t, &serial_buf, &mut send_neg);

                    if !send_neg.is_empty() {
                        port.send(&send_neg[..send_neg.len()]);
                        send_neg.clear();
                    }

                    if *flags.lock().unwrap().debug() {
                        print!("{:?}", &serial_buf[..t]);
                    }

                    if *flags.lock().unwrap().hexdump() {
                        hexdump(&serial_buf[..t]);
                    } else {
                        // Display after Coloring received string
                        if *flags.lock().unwrap().nocolor() {
                            io::stdout().write_all(output.as_bytes()).unwrap();
                        } else {
                            color::coloring_words(&output, &mut last_word, &params);
                        }
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => eprintln!("{}", e),
            }

            // Flush
            let _ = io::stdout().flush();
        }
    }
}

pub async fn transmitter<T>(mut port: T, tx: std::sync::mpsc::Sender<()>, flags: Arc<Mutex<flag::Flags>>)
where
    T: std::io::Write,
    T: self::Send,
{
    use crate::default::escape_sequences::*;

    let mut last_is_escape_signal = false;
    let g = Getch::new();

    loop {
        match g.getch() {
            Ok(key) => {
                if *flags.lock().unwrap().debug() {
                    print!("[{:?}]", key);
                    io::stdout().flush().ok();
                }

                // If the previous character is not a tilde and the current character is a tilde
                if !last_is_escape_signal && key == ESCAPE_SIGNAL {
                    last_is_escape_signal = true;
                    eprint_flush("~");
                    continue;
                }

                // Parse escape signal
                if last_is_escape_signal {
                    last_is_escape_signal = false;
                    match key {
                        ESCAPE_SIGNAL => {
                            eprint_flush("\x08");
                        },
                        EXIT_CHAR_0 | EXIT_CHAR_1 => {
                            eprint_flush(".");
                            tx.send(()).unwrap();
                            break;
                        },
                        #[cfg(not(windows))]
                        SUSPEND => {
                            use nix::unistd::Pid;
                            use nix::sys::signal::{self, Signal};

                            let _ = signal::kill(Pid::this(), Signal::SIGSTOP);
                            continue;
                        },
                        NO_COLOR => {
                            let current_nocolor = *flags.lock().unwrap().nocolor();
                            *flags.lock().unwrap().nocolor_mut() = !current_nocolor;
                            eprintln!("\x08[Changed no-color: {}]", !current_nocolor);
                            continue;
                        },
                        TIME_STAMP => {
                            let current_timestamp = *flags.lock().unwrap().timestamp();
                            *flags.lock().unwrap().timestamp_mut() = !current_timestamp;
                            eprintln!("\x08[Changed time-stamp: {}]", !current_timestamp);
                            continue;
                        },
                        INSTEAD_CRLF => {
                            let current_instead_crlf = *flags.lock().unwrap().instead_crlf();
                            *flags.lock().unwrap().instead_crlf_mut() = !current_instead_crlf;
                            eprintln!("\x08[Changed instead-crlf: {}]", !current_instead_crlf);
                            continue;
                        },
                        HEXDUMP => {
                            let current_hexdump = *flags.lock().unwrap().hexdump();
                            *flags.lock().unwrap().hexdump_mut() = !current_hexdump;
                            eprintln!("\x08[Changed hexdump mode: {}]", !current_hexdump);
                            continue;
                        },
                        DEBUG => {
                            let current_debug = *flags.lock().unwrap().debug();
                            *flags.lock().unwrap().debug_mut() = !current_debug;
                            eprintln!("\x08[Changed debug mode: {}]", !current_debug);
                            continue;
                        },
                        COMMAND_0 => {
                            eprint_flush("!");
                            if let Some(command) = echo_stdin_read_line() {
                                // Run
                                if cfg!(target_os = "windows") {
                                    Command::new("cmd").args(&["/C", &command]).spawn()
                                } else {
                                    Command::new("sh").args(&["-c", &command]).spawn()
                                }.ok();
                            }
                            continue;
                        },
                        COMMAND_1 => {
                            eprint_flush("$");
                            // Run and send
                            if let Some(command) = echo_stdin_read_line() {
                                // Run (no display)
                                let output = if cfg!(target_os = "windows") {
                                    Command::new("cmd").args(&["/C", &command]).output()
                                } else {
                                    Command::new("sh").args(&["-c", &command]).output()
                                };
                                // Send
                                if let Ok(output) = output {
                                    port.send(&output.stdout);
                                }
                            }
                            continue;
                        },
                        HELP => {
                            display_escape_sequences_help();
                            continue;
                        },
                        _ => {
                            eprintln!("\x08[Unrecognized.  Use ~~ to send ~]");
                            continue;
                        },
                    }
                }

                // If `--instead-crlf` is true, change to "\r\n"
                if *flags.lock().unwrap().instead_crlf() && key == Key::Char('\r') {
                    // Send carriage return
                    port.send(&[ b'\r', b'\n' ]);
                    continue;
                }

                // Send key
                match key {
                    Key::Null      => port.send(&[ 0x00 ]),
                    Key::Backspace => port.send(&[ 0x08 ]),
                    Key::Delete    => port.send(&[ 0x7F ]),
                    Key::Esc       => port.send(&[ 0x1B ]),
                    Key::Up        => port.send(&[ 0x1B, b'[', b'A' ]),
                    Key::Down      => port.send(&[ 0x1B, b'[', b'B' ]),
                    Key::Right     => port.send(&[ 0x1B, b'[', b'C' ]),
                    Key::Left      => port.send(&[ 0x1B, b'[', b'D' ]),
                    Key::End       => port.send(&[ 0x1B, b'[', b'F' ]),
                    Key::Home      => port.send(&[ 0x1B, b'[', b'H' ]),
                    Key::BackTab   => port.send(&[ 0x1B, b'[', b'Z' ]),
                    Key::Insert    => port.send(&[ 0x1B, b'[', b'2', b'~' ]),
                    Key::PageUp    => port.send(&[ 0x1B, b'[', b'5', b'~' ]),
                    Key::PageDown  => port.send(&[ 0x1B, b'[', b'6', b'~' ]),
                    Key::F(num) => {
                        match num {
                            v @  1..= 5 => port.send(&[ 0x1B, b'[', b'1', v + b'0',     b'~' ]),
                            v @  6..= 8 => port.send(&[ 0x1B, b'[', b'1', v + b'0' + 1, b'~' ]),
                            v @  9..=10 => port.send(&[ 0x1B, b'[', b'2', v + b'0' - 9, b'~' ]),
                            v @ 11..=12 => port.send(&[ 0x1B, b'[', b'2', v + b'0' - 8, b'~' ]),
                            _ => unreachable!(),
                        }
                    },
                    Key::Char(ch) => port.send(ch.encode_utf8(&mut [0; 4]).as_bytes()),
                    Key::Alt(ch)  => port.send(&[ 0x1B, ch as u8 ]),
                    Key::Ctrl(ch) => {
                        match ch {
                            'a'..='z' => port.send(&[ (ch as u8) - b'a' + 1 ]),
                            '4' => port.send(&[ 0x1B, b'[', b'1', b';', b'5', b'S' ]),
                            '5' => port.send(&[ 0x1B, b'[', b'1', b'5', b';', b'5', b'~' ]),
                            '6' => port.send(&[ 0x1B, b'[', b'1', b'7', b';', b'5', b'~' ]),
                            '7' => port.send(&[ 0x1B, b'[', b'1', b'8', b';', b'5', b'~' ]),
                            _ => unreachable!(),
                        }
                    },
                    Key::Other(b) => port.send(&b),
                }

            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn format_timestamp(timestamp_format: &str) -> String {
    Local::now().format(timestamp_format).to_string()
}

fn string_from_utf8_appearance(log_buf: &mut String, serial_buf: &[u8]) {
    let mut i = 0;
    while i < serial_buf.len() {
        match serial_buf[i] {
            0x8 =>  { log_buf.pop(); }        // BS
            c => {
                if c.is_ascii() {
                    (*log_buf).push(c as char);
                } else {
                    let bytes = &mut Vec::new();
                    bytes.push(serial_buf[i]);

                    for d in 1..=4 {
                        if serial_buf.len() <= i+d {
                            return;
                        }
                        bytes.push(serial_buf[i+d]);
                        if let Ok(st) = std::str::from_utf8(bytes) {
                            i += d;
                            (*log_buf).push(st.chars().next().unwrap());
                            break;
                        }
                    }
                }
            },
        }
        i += 1;
    }
}

fn display_escape_sequences_help() {
    eprintln!("?");
    eprintln!("[Escape sequences]");
    eprintln!("[~.    Drop the connection and exit]");
    eprintln!("[~^D   Drop the connection and exit]");
    eprintln!("[~^Z   Suspend (POSIX)]");
    eprintln!("[~n    Toggles the no-color]");
    eprintln!("[~t    Toggles the time-stamp]");
    eprintln!("[~i    Toggles the instead-crlf]");
    eprintln!("[~h    Toggles the hexdump mode]");
    eprintln!("[~d    Toggles the debug mode]");
    eprintln!("[~~    Send ~]");
    eprintln!("[~!    Run command in a `sh` or `cmd`]");
    eprintln!("[~$    Run command, sending the standard output]");
    eprintln!("[~?    Print this help]");
}

fn echo_stdin_read_line() -> Option<String> {
    use crate::getch;
    use rustyline::Editor;
    use lazy_static::lazy_static;

    lazy_static! { static ref RL: Mutex<Editor<()>> = Mutex::new(Editor::new()); }

    getch::enable_echo_input();
    let readline = RL.lock().unwrap().readline(">> ");
    getch::disable_echo_input();
    match readline {
        Ok(line) => {
            RL.lock().unwrap().add_history_entry(line.as_str());
            Some(line)
        },
        _ => None,
    }
}

fn eprint_flush(s: &str) {
    eprint!("{}", s);
    let _ = io::stderr().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_from_utf8_appearancering() {
        let tests = vec![
            (
                vec![0x34, 0x32],
                "42",
            ),
            (
                vec![0x08, 0x20, 0x08],
                "",
            ),
            (
                vec![0x20, 0x7e],
                " ~",
            ),
            (
                vec![0xE3, 0x81, 0x82],
                "あ",
            ),
            (
                vec![0x00],
                "\u{0}",
            ),
        ];

        for (input, expect) in tests {
            let mut buf = String::new();
            string_from_utf8_appearance(&mut buf, &input);
            assert_eq!(buf, expect);
        }
    }
}
