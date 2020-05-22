use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::time::Duration;
use std::thread;

use getch::Getch;

pub fn run(host: &str) {
    let remote = host.parse().unwrap(); // required port number

    let receiver = TcpStream::connect_timeout(&remote, Duration::from_secs(1))
        .unwrap_or_else(|e| {
            eprintln!("Could not connect: {}", e);
            std::process::exit(1);
        });
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    // Receiver
    thread::spawn(move || {
        receiver_run(receiver);
    });

    // Transmitter
    transmitter_run(transmitter);

    println!("\nDisconnected.");
}

fn receiver_run(mut port: std::net::TcpStream) {
    let mut buf: Vec<u8> = vec![0; 1024];

    loop {
        match port.read(buf.as_mut_slice()) {
            Ok(t) => {
                io::stdout().write_all(&buf[..t]).unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => eprintln!("{}", e),
        }
        io::stdout().flush().unwrap();
    }
}

fn transmitter_run(mut port: std::net::TcpStream) {
    let g = Getch::new();

    loop {
        match g.getch() {
            Ok(key) => {
                // If input "~." to exit
                if key == b'~' {
                    break;
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

// Check if the port number is attached
// If not attached, append ":23"
fn check_port(host: &str) -> String {
    todo!();
}
