use std::net::UdpSocket;
use std::{str,io};

fn main() {
	let server = ::std::env::args().nth(1).unwrap();
	let port = ::std::env::args().nth(2).unwrap();
    let my_bind = format!("{}:{}", server, port);
    let my_bind_final: &str = &my_bind;
	
	let my_port = format!("{}:8800", server);
    let socket = UdpSocket::bind(my_port).expect("bind() failed!");
    socket.connect(my_bind_final).expect("connect() failed!");
    
	loop {
        let mut input = String::new();
        let mut buffer = [0u8; 100];
        io::stdin().read_line(&mut input).expect("Could get user input!");
        socket.send(input.as_bytes()).expect("Could not send data to UDP server!");
        socket.recv_from(&mut buffer).expect("Could not get data from UDP server!");
        print!("{}", str::from_utf8(&buffer).expect("print!() failed!"));
    }
}
