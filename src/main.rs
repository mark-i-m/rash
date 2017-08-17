extern crate ssh2;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use ssh2::Session;

struct SshConnection<'a> {
    machine: &'a str,
    user: &'a str,
    pub_key: &'a str,
    priv_key: &'a str,
    tcp: Option<TcpStream>,
    sess: Option<Session>,
}

fn connect(conn: &mut SshConnection) {
    let mut tcp = TcpStream::connect(conn.machine);
    while let Err(..) = tcp {
        tcp = TcpStream::connect(conn.machine);
    }

    conn.tcp = tcp.ok();

    conn.sess = Session::new();
    conn.sess
        .as_mut()
        .unwrap()
        .handshake(conn.tcp.as_mut().unwrap())
        .unwrap();

    conn.sess
        .as_mut()
        .unwrap()
        .userauth_pubkey_file(conn.user,
                              Some(Path::new(conn.pub_key)),
                              Path::new(conn.priv_key),
                              None)
        .unwrap();
}

fn handle_special(conn: &mut SshConnection, cmd: &str) {
    match cmd {
        "##sleep" => {
            println!("{}", cmd);
            sleep(Duration::new(120, 0));
        }
        "##reconnect" => connect(conn),
        _ => println!("Command {} not recognized", cmd),
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let machine = args.next().expect("Machine name expected");
    let user = args.next().expect("User name expected");
    let script = args.next().expect("Script path expected");

    let f = File::open(script).expect("Unable to open script");
    let f = BufReader::new(f);

    let home = env::var("HOME").unwrap_or_else(|_| {
                                                   format!("/home/{}",
                env::var("USER").expect("Expected $USER or $HOME to be set"))
                                               });
    let ssh_dir = format!("{}/.ssh", home);

    let mut conn = SshConnection {
        machine: &machine,
        user: &user,
        pub_key: &format!("{}/id_rsa.pub", ssh_dir),
        priv_key: &format!("{}/id_rsa", ssh_dir),
        tcp: None,
        sess: None,
    };

    // Connect to the local SSH server
    connect(&mut conn);

    // Assume one command per line
    for cmd in f.lines() {
        let cmd = cmd.unwrap();
        let cmd = cmd.trim();

        if cmd.is_empty() {
            continue;
        }

        if cmd.chars().nth(0) == Some('#') {
            if cmd.len() > 1 && cmd.chars().nth(1) == Some('#') {
                handle_special(&mut conn, cmd);
                continue;
            } else {
                continue;
            }
        }

        let mut channel = conn.sess.as_mut().unwrap().channel_session().unwrap();

        println!("=======================");
        println!("> {}", cmd);
        channel.exec(&cmd).unwrap();

        let mut output = String::new();
        match channel.read_to_string(&mut output) {
            Ok(..) => println!("{}", output),
            Err(e) => println!("Error {:?}", e),
        }

        let mut err = String::new();
        match channel.stderr().read_to_string(&mut err) {
            Ok(..) => println!("{}", err),
            Err(e) => println!("Error {:?}", e),
        }

        match channel.exit_status() {
            Ok(e) => println!("{}", e),
            Err(e) => println!("Error {:?}", e),
        }

        match channel.close() {
            Ok(..) => channel.wait_close().unwrap(),
            Err(e) => println!("Error {:?}", e),
        }
    }
}
