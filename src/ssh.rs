use std::net::ToSocketAddrs;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::path::Path;

use anyhow;
use futures::Future;
use thrussh::*;
use thrussh_keys::*;
use tokio::net::TcpStream;

use crate::repl::{self, receiver};
use crate::flag;
use crate::setting;
use crate::getch::{Getch, Key};
use crate::default;


struct Client {
}

impl client::Handler for Client {
    type Error = anyhow::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), Self::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), Self::Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }
    fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
        println!("check_server_key: {:?}", server_public_key);
        self.finished_bool(true)
    }
    //fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
    //    println!("channel_open_confirmation: {:?}", channel);
    //    self.finished(session)
    //}
    //fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
    //    println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
    //    self.finished(session)
    //}
}

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
                dbg!(host);
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

    let config = thrussh::client::Config::default();
    let config = std::sync::Arc::new(config);
    let sh = Client{};
    //let key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
    //let mut agent = thrussh_keys::agent::client::AgentClient::connect_env().await.unwrap();
    //agent.add_identity(&key, &[]).await.unwrap();
    let mut session = thrussh::client::connect_stream(config, TcpStream::from_std(tcp_conn).unwrap(), sh).await.unwrap();


    session.authenticate_password("kumavale", "tonikakukawaii").await.unwrap();
    //if let Err(e) = session.authenticate_future(std::env::var("USER").unwrap(), key.clone_public_key(), agent).await.1 {
    //    eprintln!("3: Could not authenticate: {}", e);
    //    std::process::exit(1);
    //}

    dbg!(1);
    let mut receiver = session.channel_open_session().await.unwrap();
    receiver.request_shell(true).await.unwrap();
    // for _ in 0..2 {
    //     receiver.data(&b"Hello, world!\n"[..]).await.unwrap();
    //     dbg!("sent");
    //     if let Some(msg) = receiver.wait().await {
    //         match msg {
    //             ChannelMsg::Data{data} => {
    //                 dbg!(std::str::from_utf8(data.as_ref()));
    //             }
    //             _ => {
    //                 dbg!(format!("{:?}", msg));
    //             }
    //         }
    //     }
    // }
    //receiver.request_pty(true, "XTERM", 100, 32, 1024, 512, &[(thrussh::Pty::ECHO, 0)]).await.unwrap();
    dbg!(2);
    //  let mut transmitter = session.channel_open_session().await.unwrap();
    //  //let transmitter = Arc::clone(&receiver);
    //  transmitter.request_shell(true).await.unwrap();
    //transmitter.request_pty(true, "XTERM", 100, 32, 1024, 512, &[(thrussh::Pty::ECHO, 0)]).await.unwrap();
    //let mut transmitter: thrussh::client::Channel;
    //let mut transmitter = session.channel_open_session().await.unwrap();
    let mut transmitter = std::mem::MaybeUninit::<thrussh::client::Channel>::uninit();
    unsafe {
        //std::ptr::copy(&mut receiver as *mut _, &mut transmitter as *mut _, std::mem::size_of::<thrussh::client::Channel>());
        std::ptr::copy(&mut receiver as *mut _, transmitter.as_mut_ptr(), std::mem::size_of::<thrussh::client::Channel>());
    }
    let mut transmitter: thrussh::client::Channel = unsafe { transmitter.assume_init() };
    transmitter.request_shell(true).await.unwrap();
    dbg!(3);

    //channel.data(&b"Hello, world!"[..]).await.unwrap();
    //if let Some(msg) = channel.wait().await {
    //    println!("{:?}", msg)
    //}

    // let mut sess = Session::new().expect("Failed to create session");
    // sess.set_tcp_stream(tcp_conn.try_clone().expect("Failed to clone from receiver"));
    // sess.handshake().expect("Failed to handshake");
    // //sess.auth_methods(login_user.expect("Invalid username")).expect("Failed to authenticate user");
    // {
    //     //let mut agent = sess.agent().unwrap();
    //     //agent.connect().unwrap();
    //     //agent.list_identities().unwrap();
    //     //let identities = agent.identities().unwrap();
    //     //let identity = &identities[0];
    //     //agent.userauth(login_user.expect("Invalid username"), &identity).unwrap();
    //     sess.userauth_password(login_user.unwrap(), "toor").unwrap();
    // }
    // //debug_assert!(sess.authenticated());
    // println!("aaa");
    // let local_addr = tcp_conn.local_addr().unwrap();
    // //let local_port = receiver.local_addr().unwrap().port();
    // //let receiver = sess.clone().channel_direct_tcpip(host, 22, None).unwrap();
    // //let mut receiver = sess.clone();//.channel_session().unwrap();
    // //let receiver = sess.clone().channel_direct_tcpip(&local_addr.ip().to_string(), local_addr.port(), None).unwrap();
    // //let receiver = sess.channel_session().unwrap().stream(FLUSH_ALL);
    // let mut receiver = sess.clone().channel_session().unwrap();
    // receiver.request_pty(&terminal_type, None, None).unwrap();
    // //let mut mode = ssh2::PtyModes::new();
    // //mode.set_character(ssh2::PtyModeOpcode::ECHO, None);
    // //receiver.request_pty(&terminal_type, Some(mode), None).unwrap();
    // receiver.shell().unwrap();
    // let receiver_tmp = receiver.stream(0);
    // //let receiver = tcp_conn.try_clone().unwrap(); // receiver.stream(0);
    // //let receiver = sess.channel_direct_tcpip(&local_addr.ip().to_string(), 22, None).unwrap();
    // //receiver.request_pty(&terminal_type, None, None).unwrap();
    // //receiver.shell().unwrap();
    // println!("bbb");
    // //let transmitter = receiver.try_clone().unwrap();
    // let transmitter = receiver.stream(0);
    // let receiver = receiver_tmp;
    // //let transmitter = receiver.stream(0);
    // //let transmitter = sess.channel_session().unwrap().stream(FLUSH_ALL);
    // //let mut transmitter = sess.channel_session().unwrap();
    // //let transmitter = sess.channel_direct_tcpip(&local_addr.ip().to_string(), 22, None).unwrap();
    // //let transmitter = receiver.stream(0);
    // //let transmitter = receiver.stream(dbg!(FLUSH_ALL));
    // //transmitter.shell().unwrap();
    // //let transmitter = sess.channel_session().unwrap();//.stream(0);
    // //let transmitter = sess;
    // //let mut transmitter = sess.channel_session().unwrap();
    // //////transmitter.request_pty(&terminal_type, None, None).unwrap();
    // //let mut mode = ssh2::PtyModes::new();
    // //mode.set_character(ssh2::PtyModeOpcode::ECHO, None);
    // //transmitter.request_pty(&terminal_type, Some(mode), None).unwrap();
    // //let transmitter = transmitter.stream(0);
    // //transmitter.shell().unwrap();
    // //let transmitter = tcp_conn;
    // //let transmitter = sess.channel_direct_tcpip(&local_addr.ip().to_string(), local_addr.port(), None).unwrap();
    // //let mut transmitter = sess;//.channel_session().expect("Failed to create channel from session");
    // //let transmitter = sess.clone().channel_direct_tcpip(host, 22, None).unwrap();
    // //transmitter.request_pty(&terminal_type, None, None).unwrap();
    // //transmitter.shell().unwrap();
    // println!("ccc");

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
        _ = tokio::spawn(repl::receiver_ssh(receiver, rx, flags_clone, params)) => {
            println!("\n\x1b[0mDisconnected.");
            std::process::exit(0);
        }
        _ = tokio::spawn(repl::transmitter_ssh(transmitter, tx, flags)) => {}
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
            assert_eq!(to_SocketAddr_for_ssh(&input), expect);
        }
        for (input, expect, or_expect) in tests_hostname {
            assert!(
                to_SocketAddr_for_ssh(&input) == expect
             || to_SocketAddr_for_ssh(&input) == or_expect
            );
        }
    }
}
