extern crate hyper;
extern crate rand;
use rand::Rng;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

fn hello(_: Request, res: Response) {
    let number = rand::thread_rng().gen_range(0, 100);
    let data = format!("{}\r\n", number);
	res.send(data.as_bytes()).expect("Failed to send data!");
}

fn main() {
	let port = ::std::env::args().nth(1).unwrap();
	let ip_and_port = "127.0.0.1".to_string() + ":" + &port;
	let ip_port: &str = &ip_and_port;
    Server::http(ip_port).unwrap().handle(hello).unwrap();
}
