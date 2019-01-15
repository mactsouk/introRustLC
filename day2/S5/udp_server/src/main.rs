extern crate rand;
use rand::Rng;

use std::str;
use std::thread;
use std::net::UdpSocket;

fn main() {
    let port = ::std::env::args().nth(1).unwrap();
    // localhost
    let ip = "127.0.0.1";
    let my_bind = ip.to_string() + ":" + &port;
    let my_bind_final: &str = &my_bind;

    let socket = match UdpSocket::bind(my_bind_final) {
        Ok(s) => s,
        Err(e) => panic!("Could not bind socket: {}", e)
    };

    let mut buf = [0; 100];
    loop {
        let sock = socket.try_clone().expect("Failed to clone socket!");
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("Got {} bytes from {}.", amt, src);
                    print!("{}", str::from_utf8(&buf[0..amt]).unwrap_or(""));
                    let number = rand::thread_rng().gen_range(0, 100);
                    let data = format!("{}\r\n", number);
                    sock.send_to(data.as_bytes(), &src).expect("Failed to send a response");
                });
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}
