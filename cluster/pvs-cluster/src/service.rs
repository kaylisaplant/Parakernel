use std::net::{TcpStream};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub service_addr: Vec<String>,
    pub service_port: i32,
    pub service_claim: u64,
    pub interface_addr: Vec<String>
}

pub fn serialize(payload: & Payload) -> String {
    serde_json::to_string(payload).unwrap()
}

pub fn request_handler(stream: & mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(& mut buffer)?;

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}