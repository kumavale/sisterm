use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::path::Path;

use crate::repl;
use crate::flag;
use crate::setting;
use crate::getch::{Getch, Key};
use crate::default;

pub fn run(host:      &str,
           mut flags: flag::Flags,
           params:    Option<setting::Params>)
{
    let tcp_connect_timeout = params.as_ref().map_or_else(|| default::TCP_CONNECT_TIMEOUT, |p| p.tcp_connect_timeout);

    let receiver = {
        let hosts = to_SocketAddr(host);
        if hosts.is_empty() {
            eprintln!("Could not connect: {}", host);
            std::process::exit(1);
        } else {
            let mut result = None;
            for host in hosts {
                let r = TcpStream::connect_timeout(&host, Duration::from_secs(tcp_connect_timeout));
                if r.is_ok() {
                    result = Some(r);
                    break;
                }
            }
            result.unwrap_or_else(|| {
                eprintln!("Could not connect: {}", host);
                std::process::exit(1);
            }).unwrap_or_else(|e| {
                eprintln!("Could not connect: {}", e);
                std::process::exit(1);
            })
        }
    };
    receiver.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    let (tx, rx) = mpsc::channel();

    // If write_file is already exists
    if let Some(write_file) = flags.write_file() {
        if Path::new(&write_file).exists() {
            if !*flags.append() {
                let g = Getch::new();
                println!("\"{}\" is already exists!", &write_file);
                println!("Press ENTER to continue overwrite");
                match g.getch() {
                    Ok(Key::Char('\r')) => (),   // continue
                    _ => std::process::exit(0),  // exit
                }
            }
        } else if *flags.append() {
            let g = Getch::new();
            println!("\"{}\" is not exists!", &write_file);
            println!("Press ENTER to create the file and continue");
            match g.getch() {
                Ok(Key::Char('\r')) => (),   // continue
                _ => std::process::exit(0),  // exit
            }
            *flags.append_mut() = false;
        }
    }

    // Check if params exists
    if params.is_none() {
        *flags.nocolor_mut() = true;
    }

    println!("Type \"~.\" to exit.");
    println!("Connecting... {}", host);

    let flags       = Arc::new(Mutex::new(flags));
    let flags_clone = flags.clone();

    // Receiver
    let handle = thread::spawn(move || {
        repl::receiver(receiver, rx, flags_clone, params);

        println!("\n\x1b[0mDisconnected.");
        std::process::exit(0);
    });

    // Transmitter
    repl::transmitter(transmitter, tx, flags);

    handle.join().unwrap();
}

// Check if the port number is attached
#[allow(non_snake_case)]
fn to_SocketAddr(host: &str) -> Vec<std::net::SocketAddr> {
    host.to_socket_addrs().unwrap().collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    #[allow(non_snake_case)]
    fn test_to_SocketAddr() {
        let tests_ipaddr = vec![
            (
                "127.0.0.1:23",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23))],
            ),
            (
                "127.0.0.1:12321",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 12321))],
            ),
            (
                "::1:23",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 23, 0, 0))],
            ),
            (
                "[::1]:23",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 23, 0, 0))],
            ),
            (
                "[::1]:12321",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 12321, 0, 0))],
            ),
        ];

        let tests_hostname = vec![
            (
                "localhost:23",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23))],
                vec![
                    SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 23, 0, 0)),
                    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23)),
                ],
            ),
        ];

        for (input, expect) in tests_ipaddr {
            assert_eq!(to_SocketAddr(input), expect);
        }
        for (input, expect, or_expect) in tests_hostname {
            assert!(
                to_SocketAddr(input) == expect
             || to_SocketAddr(input) == or_expect
            );
        }
    }

    #[test]
    #[should_panic]
    #[allow(non_snake_case)]
    fn should_fail_to_SocketAddr() {
        let tests_ipaddr = vec![
            (
                "127.0.0.1",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23))],
            ),
        ];

        for (input, expect) in tests_ipaddr {
            assert_eq!(to_SocketAddr(input), expect);
        }
    }
}
