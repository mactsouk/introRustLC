extern crate rand;
use rand::Rng;

use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle_request(mut stream: TcpStream) {
    print!("Sending random numbers!\r\n");

    let mut buf;
    loop {
        buf = [0; 1024];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
            },
        };
		let number = rand::thread_rng().gen_range(0, 100);
	    let data = format!("{}\r\n", number);
        stream.write(data.as_bytes()).unwrap();
    }
}

fn main() {
    let port = ::std::env::args().nth(1).unwrap();
    let ip = "127.0.0.1";
    let my_bind = ip.to_string() + ":" + &port;
    let my_bind_final: &str = &my_bind;
    
    let listener = TcpListener::bind(my_bind_final).unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("Failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_request(stream)
                });
            }
        }
    }
}
