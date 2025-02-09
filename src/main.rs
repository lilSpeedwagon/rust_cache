use std::{io::Read, net::TcpListener};
use std::io::{prelude::*, BufReader};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let buf_reader = BufReader::new(&stream);
        let lines: Vec<String> = buf_reader.lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect();
        for l in lines {
            println!("{}", l);
        }

        let buffer = String::from("Hello");
        stream.write(&buffer.as_bytes()).unwrap();
    }
}
