use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct Client {
    name: String,
    address: String,
}

impl Client {
    pub fn new(name: String, address: String) -> Self {
        Client {
            name: name,
            address: address,
        }
    }
    pub fn connect(&self) -> std::io::Result<()> {
        let mut name: String = String::new();
        println!("trying to connect to {}", self.address);
        let mut stream =
            TcpStream::connect(self.address.clone()).expect("Couldn't connect to server...");

        stream.write(format!("{}", self.name).as_bytes())?;

        stream.read_to_string(&mut name)?;

        println!("server responded: {}", name);
        Ok(())
    }
}
