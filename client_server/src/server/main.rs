use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream);

        stream
            .shutdown(std::net::Shutdown::Both)
            .unwrap_or_else(|e| println!("Failed to shutdown because: {e}"));
    }
}

fn handle_connection(stream: &TcpStream) {
    let reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream.try_clone().unwrap();

    let s = reader.lines().next().unwrap().unwrap();

    match s.as_str() {
        "foo" => {
            println!("Received foo message");
            writer.write_all("got your foo, thanks".as_bytes()).unwrap();
            writer.flush().unwrap();
        }
        msg => println!(r#"Received some other message: "{msg}""#),
    }
}
