use std::io::{Read, Write, Result};
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

    pub fn claim(&mut self, k:u64) ->Result {

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

pub fn deserialize(payload: & String) -> Payload {
    serde_json::from_str(payload).unwrap()
}

fn stream_read(stream: & mut TcpStream) -> Result<String>{
    let mut buf = [0; 1024];
    let mut message = String::new();

    loop {
        let bytes_read = stream.read(&mut buf)?;
        let s = std::str::from_utf8(&buf[..bytes_read]).unwrap();
        message.push_str(s);

        if bytes_read < buf.len() {
            break;
        }
    }

    Ok(message)
}

pub fn request_handler(
    state: & mut State, stream: & mut TcpStream
) -> std::io::Result<()> {

    let payload = match stream_read(stream) {
        Ok(message) => deserialize(& message),
        Err(message) => panic!("Encountered error {}", message)
    };

    println!("Reqest processed: {:?}", payload);
    if state.clients.contains_key(& payload.key){

    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    println!("Now state:");
    state.print();

    Ok(())
}