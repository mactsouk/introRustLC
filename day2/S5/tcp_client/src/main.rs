use std::net::TcpStream;
use std::time::Duration;

use std::io::Write;
use std::io::Read;

fn main() {
	let one_second = Duration::new(1, 0);
    let server = ::std::env::args().nth(1).unwrap();
    let port = ::std::env::args().nth(2).unwrap();
	
    let my_bind = format!("{}:{}", server, port);
    let my_bind_final: &str = &my_bind;
	
    let mut buf: [u8; 20] = [0; 20];
    let stream = TcpStream::connect(my_bind_final);
	std::thread::sleep(one_second);
    println!("stream info: {:?}", stream);

	let mut my_stream = stream.unwrap();
    my_stream.write(b"Hello there!\r\n").unwrap();
    let _ = match my_stream.read(&mut buf) {
        Err(e) => panic!("Error: {}", e),
        Ok(m) => {
            if m == 0 {
                println!("Failed to Read!");
            }
			println!("Read: {} chars", m);
            m
        },
    };

    let s = String::from_utf8_lossy(&buf);
    print!("Read: {}", s);
}
