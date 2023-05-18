use std::{
    io::{BufWriter, Write, Read},
    net::TcpStream,
};

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:7878").unwrap();

    let mut writer = BufWriter::new(client.try_clone().unwrap());
    writer.write_all("bar\n".as_bytes()).unwrap();
    writer.flush().unwrap();

    let mut s = String::new();
    client.read_to_string(&mut s).unwrap();

    println!(r#"Server responded with: "{s}""#);
}
