use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address: address }
    }
    pub fn serve(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address.clone())?;
        println!("tcp server listening at: {}", self.address);
        for stream in listener.incoming() {
            handle_client(stream?);
        }
        Ok(())
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut name: Vec<u8> = vec![];
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let count = reader.read_until(b'\n', &mut name).unwrap();
    let name = String::from_utf8_lossy(&name);
    if count > 0 {
        println!("Hello, {}!", name);
    }
    stream
        .write_all(format!("Hello, {}!", name).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}
