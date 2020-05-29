use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::path::Path;

use crate::repl;
use crate::flag;
use crate::setting;
use crate::getch::Getch;
use crate::default;
use crate::negotiation;

pub fn run(host:      &str,
           mut flags: flag::Flags,
           params:    Option<setting::Params>)
{
    let tcp_connect_timeout = if let Some(ref p) = params {
        p.tcp_connect_timeout
    } else {
        default::TCP_CONNECT_TIMEOUT
    };

    let receiver = {
        let hosts = to_SocketAddr_for_telnet(host);
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
    let mut transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    let (tx, rx) = mpsc::channel();

    // If write_file is already exists
    if let Some(write_file) = flags.write_file() {
        if Path::new(write_file).exists() {
            if !flags.is_append() {
                let g = Getch::new();
                println!("\"{}\" is already exists!", write_file);
                println!("Press ENTER to continue overwrite");
                match g.getch() {
                    Ok(b'\n') | Ok(b'\r') => (),  // continue
                    _ => std::process::exit(0),   // exit
                }
            }
        } else if flags.is_append() {
            let g = Getch::new();
            println!("\"{}\" is not exists!", write_file);
            println!("Press ENTER to create the file and continue");
            match g.getch() {
                Ok(b'\n') | Ok(b'\r') => (),  // continue
                _ => std::process::exit(0),   // exit
            }
            flags.set_append(false);
        }
    }

    // Check if params exists
    if params.is_none() {
        flags.set_nocolor(true);
    }

    // The first negotiation
    negotiation::init(&mut transmitter);

    println!("Connected. {}:", host);
    println!("Type \"~.\" to exit.");

    // Receiver
    let flags_clone = flags.clone();
    let handle = thread::spawn(move || {
        repl::receiver_telnet(receiver, rx, flags_clone, params);

        println!("\n\x1b[0mDisconnected.");
        std::process::exit(0);
    });

    // Transmitter
    repl::transmitter_telnet(transmitter, tx, flags);

    handle.join().unwrap();
}

// Check if the port number is attached
// If not attached, append ":23"
#[allow(non_snake_case)]
fn to_SocketAddr_for_telnet(host: &str) -> Vec<std::net::SocketAddr> {
    match host.to_socket_addrs() {
        Ok(result) => result.collect(),
        Err(_) => {
            let mut host = host.to_string();
            host.push_str(":23");
            host.to_socket_addrs().unwrap().collect()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    #[allow(non_snake_case)]
    fn test_to_SocketAddr_for_telnet() {
        let tests_ipaddr = vec![
            (
                "127.0.0.1",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23))],
            ),
            (
                "127.0.0.1:23",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23))],
            ),
            (
                "127.0.0.1:12321",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 12321))],
            ),
        ];

        let tests_hostname = vec![
            (
                "localhost",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23))],
                vec![
                    SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 23, 0, 0)),
                    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23)),
                ],
            ),
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
            assert_eq!(to_SocketAddr_for_telnet(&input), expect);
        }
        for (input, expect, or_expect) in tests_hostname {
            assert!(
                to_SocketAddr_for_telnet(&input) == expect
             || to_SocketAddr_for_telnet(&input) == or_expect
            );
        }
    }
}
