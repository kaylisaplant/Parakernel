use std::io::{Read, Write};
use std::net::TcpStream;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub service_addr: Vec<String>,
    pub service_port: i32,
    pub service_claim: u64,
    pub interface_addr: Vec<String>,
    pub key: u64
}

#[derive(Debug)]
pub struct State {
    pub clients: HashMap<u64, Vec<Payload>>,
}

impl State {
    pub fn new() -> State {
        State{
            clients: HashMap::new()
        }
    }
    pub fn add(&mut self, p: Payload) {
        let cl: &mut Vec<Payload> = self.clients.entry(p.key).or_insert(Vec::new());
        cl.push(p);
    }

    pub fn print(&mut self) {
        for (key, values) in & self.clients {
            for v in values {
                println!("{}: {:?}", key, v);
            }
        }
    }
}

pub fn serialize(payload: & Payload) -> String {
    serde_json::to_string(payload).unwrap()
}

pub fn request_handler(
    state: & mut State, stream: & mut TcpStream
) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(& mut buffer)?;

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    println!("Reqest processed");
    state.print();

    Ok(())
}