use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub struct Addr<'a> {
    pub host: &'a String,
    pub port: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub header: u8,
    pub body: String
}


pub fn serialize_message(payload: & Message) -> String {
    serde_json::to_string(payload).unwrap()
}


pub fn deserialize_message(payload: & String) -> Message {
    serde_json::from_str(payload).unwrap()
}


pub fn cwrite(addr: & str, port: i32, msg: & str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", addr, port))?;

    stream.write_all(msg.as_bytes())?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received response: {}", response);

    Ok(())
}


pub fn stream_read(stream: & mut TcpStream) -> std::io::Result<String>{
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


pub fn server(
    addr: &Addr, 
    mut handler: impl FnMut(&mut TcpStream) -> std::io::Result<()>
) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", addr.host, addr.port))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // pass the handle_connection function as a function pointer
                handler(&mut stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}