use crate::connection::{cread, cwrite};

use serde::{Serialize, Deserialize};
// use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub service_addr: Vec<String>,
    pub service_port: i32,
    pub service_claim: u64,
    pub interface_addr: Vec<String>
}

pub fn serialize(payload: & Payload) -> String {
    serde_json::to_string(payload).unwrap()
}

// fn handle_connection(stream: & mut TcpStream) -> std::io::Result<()> {
//     let mut buffer = [0; 512];
//     stream.read(&mut buffer)?;
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     stream.write(response.as_bytes())?;
//     stream.flush()?;
//     Ok(())
// }
// 
// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:8080")?;
//     for stream in listener.incoming() {
//         let mut stream = stream?;
//         handle_connection(&mut stream)?;
//     }
//     Ok(())
// }
// 