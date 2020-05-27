use std::io::{self, BufWriter, Write};
use std::fs::OpenOptions;

use crate::queue::Queue;
use crate::flag;
use crate::color;
use crate::setting;
use crate::getch::Getch;

use chrono::Local;


pub fn receiver_run<T>(
    mut port: T,
    rx:       std::sync::mpsc::Receiver<()>,
    flags:    flag::Flags,
    params:   Option<setting::Params>)
where
    T: std::io::Read,
{
    let buf_size = flags.buf_size();
    dbg!(buf_size);
    let mut serial_buf: Vec<u8> = vec![0; buf_size];
    let mut last_word  = (String::new(), false);  // (increasing_str, prev_matched)

    // Save log
    if let Some(write_file) = flags.write_file() {

        let mut log_file   = {
            if flags.is_append() {
                BufWriter::new(OpenOptions::new()
                    .append(true)
                    .open(write_file)
                    .expect("File open failed"))
            } else {
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(write_file)
                    .expect("File open failed"))
            }
        };
        let mut log_buf    = String::new();
        let mut write_flag = false;

        println!("Log record: \"{}\" ({})\n", write_file,
            if flags.is_append() {
                "Append"
            } else {
                "Overwrite"
            }
        );

        loop {
            // if "~." is typed, exit
            if rx.try_recv().is_ok() {
                log_file.write_all(log_buf.as_bytes()).unwrap();
                log_file.flush().unwrap();
                break;
            }

            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // Display after Coloring received string
                    if flags.is_nocolor() {
                        let buf = erase_control_char(&serial_buf[..t]);
                        io::stdout().write_all(buf.as_bytes()).unwrap();
                    } else {
                        let buf = erase_control_char(&serial_buf[..t]);
                        color::coloring_words(&buf, &mut last_word, &params);
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
                if flags.is_timestamp() {
                    // If '\n' exists, replace to timestamp from '\n'
                    log_buf = log_buf.replace("\n", &format_timestamp());
                }
                log_file.write_all(log_buf.as_bytes()).unwrap();
                log_buf.clear();
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
                    // Display after Coloring received string
                    if flags.is_nocolor() {
                        let buf = erase_control_char(&serial_buf[..t]);
                        io::stdout().write_all(buf.as_bytes()).unwrap();
                    } else {
                        let buf = erase_control_char(&serial_buf[..t]);
                        color::coloring_words(&buf, &mut last_word, &params);
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

pub fn transmitter_run<T>(mut port: T, tx: std::sync::mpsc::Sender<()>, flags: flag::Flags)
where
    T: std::io::Write,
{
    let exit_char1 = b'~';
    let exit_char2 = b'.';
    let mut queue = Queue::new(exit_char1, exit_char2);
    let mut last_is_tilde = false;
    let g = Getch::new();

    loop {
        match g.getch() {
            Ok(key) => {
                // Arrow keys
                if cfg!(windows) && key == 0 || key == 224 { // ESC
                    match g.getch() {
                        Ok(72) => if let Err(e) = port.write(&[27, 91, 65]) { eprintln!("{}", e); }, // Up
                        Ok(80) => if let Err(e) = port.write(&[27, 91, 66]) { eprintln!("{}", e); }, // Down
                        Ok(77) => if let Err(e) = port.write(&[27, 91, 67]) { eprintln!("{}", e); }, // Right
                        Ok(75) => if let Err(e) = port.write(&[27, 91, 68]) { eprintln!("{}", e); }, // Left
                        Ok(k)  => if let Err(e) = port.write(&[224, k])     { eprintln!("{}", e); }, // Other
                        Err(e) => eprintln!("{}", e),
                    }
                    continue;
                }
                if cfg!(not(windows)) && key == 27 { // ESC
                    let _ = g.getch();
                    match g.getch() {
                        Ok(b'A') => if let Err(e) = port.write(&[27, 91, 65]) { eprintln!("{}", e); }, // Up
                        Ok(b'B') => if let Err(e) = port.write(&[27, 91, 66]) { eprintln!("{}", e); }, // Down
                        Ok(b'C') => if let Err(e) = port.write(&[27, 91, 67]) { eprintln!("{}", e); }, // Right
                        Ok(b'D') => if let Err(e) = port.write(&[27, 91, 68]) { eprintln!("{}", e); }, // Left
                        Ok(k)    => if let Err(e) = port.write(&[27, 91, k])  { eprintln!("{}", e); }, // Other
                        Err(e) => eprintln!("{}", e),
                    }
                    continue;
                }

                queue.enqueue(key);
                // If input "~." to exit
                if queue.is_exit_chars() {
                    eprint!(".");
                    let _ = io::stdout().flush();
                    tx.send(()).unwrap();
                    break;
                }
                // If the previous character is not a tilde and the current character is a tilde
                if !last_is_tilde && key == exit_char1 {
                    last_is_tilde = true;
                    eprint!("~");
                    let _ = io::stdout().flush();
                    continue;
                }
                // If not input "~~" to dispaly error message
                if last_is_tilde {
                    if key == exit_char1 {
                        eprint!("\x08");
                        queue.enqueue(0);
                    } else {
                        eprint!("\x08");
                        eprintln!("[Unrecognized.  Use ~~ to send ~]");
                        let _ = io::stdout().flush();
                        last_is_tilde = false;
                        continue;
                    }
                }
                last_is_tilde = false;

                if flags.is_instead_cr() && key == b'\n' {
                    // Send carriage return
                    if let Err(e) = port.write(&[b'\r']) {
                        eprintln!("{}", e);
                    }
                    continue;
                }

                // Send key
                if let Err(e) = port.write(&[key]) {
                    eprintln!("{}", e);
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn format_timestamp() -> String {
    Local::now().format("\n[%Y-%m-%d %H:%M:%S %Z] ").to_string()
}

fn string_from_utf8_appearance(log_buf: &mut String, serial_buf: &[u8]) {
    for c in serial_buf {
        match *c {
            0x8 => { log_buf.pop(); }      // BS
            0x9 => (*log_buf).push('\t'),  // HT
            0xa => (*log_buf).push('\n'),  // LF
            0xd => (*log_buf).push('\r'),  // CR
            c => {
                if 0x20 <= c && c <= 0x7e {
                    (*log_buf).push(c as char);
                }
                // Ignore others
            },
        }
    }
}

fn erase_control_char(serial_buf: &[u8]) -> String {
    let mut buf = String::new();
    for c in serial_buf {
        match *c {
            0x8 => (buf).push(*c as char),  // BS
            0x9 => (buf).push(*c as char),  // HT
            0xa => (buf).push(*c as char),  // LF
            0xd => (buf).push(*c as char),  // CR
            c => {
                if 0x20 <= c && c <= 0x7e {
                    (buf).push(c as char);
                }
                // Ignore others
            },
        }
    }
    buf
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_from_utf8_appearancering() {
        let tests = vec![
            (
                [0x34, 0x32],
                "42",
            ),
            (
                [0x00, 0x07],
                "",
            ),
            (
                [0x1f, 0x7f],
                "",
            ),
            (
                [0x08, 0x08],
                "",
            ),
            (
                [0x20, 0x7e],
                " ~",
            ),
        ];

        for (input, expect) in tests {
            let mut buf = String::new();
            string_from_utf8_appearance(&mut buf, &input);
            assert_eq!(buf, expect);
        }
    }

    #[test]
    fn test_erase_control_char() {
        let tests = vec![
            (
                [0x34, 0x32],
                "42",
            ),
            (
                [0x00, 0x07],
                "",
            ),
            (
                [0x1f, 0x7f],
                "",
            ),
            (
                [0x08, 0x08],
                "\x08\x08",
            ),
            (
                [0x20, 0x7e],
                " ~",
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(erase_control_char(&input), expect);
        }
    }
}
