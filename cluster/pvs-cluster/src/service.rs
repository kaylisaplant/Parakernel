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