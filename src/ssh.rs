use std::convert::TryInto;
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::path::Path;

use ssh2::Session;

use crate::repl;
use crate::flag;
use crate::setting;
use crate::getch::{Getch, Key};
use crate::default;
use crate::negotiation::get_window_size;

pub async fn run(
    host:       &str,
    mut flags:  flag::Flags,
    params:     Option<setting::Params>,
    login_user: Option<&str>,
){
    let tcp_connect_timeout = params.as_ref().map_or_else(|| default::TCP_CONNECT_TIMEOUT, |p| p.tcp_connect_timeout);
    let terminal_type       = params.as_ref().map_or_else(|| default::TERMINAL_TYPE,       |p| &p.terminal_type);

    let tcp_conn = {
        let hosts = to_SocketAddr_for_ssh(host);
        if hosts.is_empty() {
            eprintln!("1: Could not connect: {}", host);
            std::process::exit(1);
        } else {
            let mut result = None;
            for host in hosts {
                let r = std::net::TcpStream::connect_timeout(&host, Duration::from_secs(tcp_connect_timeout));
                if r.is_ok() {
                    result = Some(r);
                    break;
                }
            }
            result.unwrap_or_else(|| {
                eprintln!("2: Could not connect: {}", host);
                std::process::exit(1);
            }).unwrap_or_else(|e| {
                eprintln!("3: Could not connect: {}", e);
                std::process::exit(1);
            })
        }
    };

    tcp_conn.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

    let mut sess = Session::new().expect("Failed to create session");
    sess.set_tcp_stream(tcp_conn);
    sess.handshake().expect("Failed to handshake");

    {
        // Authentication
        for _ in 0..default::RETRY_AUTH_COUNT {
            use std::io::prelude::*;
            let user = if let Some(user) = login_user {
                user.to_string()
            } else {
                print!("username: ");
                let _ = std::io::stdout().flush();
                let mut user = String::new();
                std::io::stdin().read_line(&mut user).unwrap();
                user.trim().to_string()
            };
            let pass = rpassword::prompt_password("password: ").unwrap();

            if let Err(e) = sess.userauth_password(&user, &pass) {
                eprintln!("{}", e);
            } else {
                break;
            }
        }
    }

    let mut channel = sess.channel_session().unwrap();
    let (width, height) = terminal_size();
    channel.request_pty(terminal_type, None, Some((width, height, 0, 0))).unwrap();
    channel.shell().unwrap();

    let receiver    = channel.stream(0);
    let transmitter = channel.stream(0);
    sess.set_blocking(false);

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

    tokio::select! {
        _ = tokio::spawn(repl::receiver(receiver, rx, flags_clone, params)) => {
            println!("\n\x1b[0mDisconnected.");
            std::process::exit(0);
        }
        _ = tokio::spawn(repl::transmitter(transmitter, tx, flags)) => {}
    }
}

// Check if the port number is attached
// If not attached, append ":22"
#[allow(non_snake_case)]
fn to_SocketAddr_for_ssh(host: &str) -> Vec<std::net::SocketAddr> {
    match host.to_socket_addrs() {
        Ok(result) => result.collect(),
        Err(_) => {
            let mut host = host.to_string();
            host.push_str(":22");
            host.to_socket_addrs().unwrap().collect()
        }
    }
}

// return terminal size
fn terminal_size() -> (u32, u32) {
    let window_size = get_window_size();
    let width  = u16::from_be_bytes(window_size[..2].try_into().unwrap()) as u32;
    let height = u16::from_be_bytes(window_size[2..].try_into().unwrap()) as u32;
    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    #[allow(non_snake_case)]
    fn test_to_SocketAddr_for_ssh() {
        let tests_ipaddr = vec![
            (
                "127.0.0.1",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22))],
            ),
            (
                "127.0.0.1:22",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22))],
            ),
            (
                "127.0.0.1:12321",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 12321))],
            ),
            (
                "::1",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 22, 0, 0))],
            ),
            (
                "::1:22",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 22, 0, 0))],
            ),
            (
                "[::1]:22",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 22, 0, 0))],
            ),
            (
                "[::1]:12321",
                vec![SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 12321, 0, 0))],
            ),
        ];

        let tests_hostname = vec![
            (
                "localhost",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22))],
                vec![
                    SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 22, 0, 0)),
                    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22)),
                ],
            ),
            (
                "localhost:22",
                vec![SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22))],
                vec![
                    SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 22, 0, 0)),
                    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22)),
                ],
            ),
        ];

        for (input, expect) in tests_ipaddr {
            assert_eq!(to_SocketAddr_for_ssh(input), expect);
        }
        for (input, expect, or_expect) in tests_hostname {
            assert!(
                to_SocketAddr_for_ssh(input) == expect
             || to_SocketAddr_for_ssh(input) == or_expect
            );
        }
    }
}
