use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:7878").unwrap();

    client.write_all("foo".as_bytes()).unwrap();

    let mut s = String::new();
    client.read_to_string(&mut s).unwrap();

    println!(r#"Server responded with: "{s}""#);
}
