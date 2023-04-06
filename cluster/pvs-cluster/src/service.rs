use serde::{Serialize, Deserialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    service_addr: Vec<String>,
    service_port: i32,
    service_claim: u64
}

pub fn serialzie(payload: &Payload) {
    serde_json::to_string(payload).unwrap()
}