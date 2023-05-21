use std::io::Write;
use std::net::TcpStream;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::utils::epoch;
use crate::connection::{Message, stream_read, serialize_message, deserialize_message};


#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub service_addr: Vec<String>,
    pub service_port: i32,
    pub service_claim: u64,
    pub interface_addr: Vec<String>,
    pub key: u64,
    pub id: u64
}


#[derive(Debug)]
pub struct State {
    pub clients: HashMap<u64, Vec<Payload>>,
    pub timeout: u64,
    pub seq: u64
}


impl State {
    pub fn new() -> State {
        State{
            clients: HashMap::new(),
            timeout: 60,
            seq: 1
        }
    }

    pub fn add(&mut self, mut p: Payload) {
        let cl: &mut Vec<Payload> = self.clients.entry(p.key).or_insert(Vec::new());
        p.id = self.seq;
        cl.push(p);
        self.seq += 1;
    }

    pub fn claim(&mut self, k:u64) -> Result<&mut Payload, u64> {
        match self.clients.get_mut(& k) {

            Some(value) => for v in value {
                let current_ecpoch = epoch();
                if current_ecpoch - v.service_claim > self.timeout {
                    v.service_claim = current_ecpoch;
                    return Ok(v);
                }
            }

            _ => return Err(1)
        }

        return Err(2);
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


pub fn heartbeat_handler(stream: & mut TcpStream) -> std::io::Result<()> {

    let request = match stream_read(stream) {
        Ok(message) => deserialize_message(& message),
        Err(message) => panic!("Encountered error {}", message)
    };

    if request.header != 0 {
        panic!(
            "Non-heartbeat request {} sent to heartbeat_handler: {}",
            request.header, request.body
        );
    }

    let response = serialize_message(& request);
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}