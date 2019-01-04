use std::net::IpAddr;

fn main() {
    let my_ip = "127.0.0.1";
    let parsed_ip: IpAddr = my_ip.parse().unwrap();
    println!("{}", parsed_ip);
    
    let invalid_ip = "727.0.0.1";
    let try_parsed_ip: IpAddr = invalid_ip.parse().unwrap();
    println!("{}", try_parsed_ip);
}
