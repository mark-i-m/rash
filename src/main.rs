extern crate ssh2;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use ssh2::Session;

struct SshConnection<'a> {
    machine: &'a str,
    user: &'a str,
    tcp: TcpStream,
    sess: Session,
}

fn handle_special(conn: &mut SshConnection, cmd: &str) {
    if cmd == "##sleep" {
        println!("{}", cmd);
        sleep(Duration::new(120, 0));
    } else if cmd == "##reconnect" {
        let mut tcp = TcpStream::connect(conn.machine);
        while let Err(..) = tcp {
            tcp = TcpStream::connect(conn.machine);
        }

        conn.tcp = tcp.unwrap();

        conn.sess = Session::new().unwrap();
        conn.sess.handshake(&conn.tcp).unwrap();

        conn.sess.userauth_pubkey_file(conn.user,
                                  Some(Path::new("/afs/cs.wisc.edu/u/m/a/markm/.ssh/id_rsa.pub")),
                                  Path::new("/afs/cs.wisc.edu/u/m/a/markm/.ssh/id_rsa"),
                                  None).unwrap();
        println!("Reconnected!");
    }

    println!("=======================");
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let machine = args.next().expect("Machine name expected");
    let user = args.next().expect("User name expected");
    let script = args.next().expect("Script path expected");

    let f = File::open(script).expect("Unable to open script");
    let f = BufReader::new(f);

    // Connect to the local SSH server
    let tcp = TcpStream::connect(&machine).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();

    sess.userauth_pubkey_file(&user,
                              Some(Path::new("/afs/cs.wisc.edu/u/m/a/markm/.ssh/id_rsa.pub")),
                              Path::new("/afs/cs.wisc.edu/u/m/a/markm/.ssh/id_rsa"),
                              None).unwrap();

    let mut conn = SshConnection {tcp, sess, user: &user, machine: &machine};

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

        let mut channel = conn.sess.channel_session().unwrap();

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

        println!("=======================");
    }
}
