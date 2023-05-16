use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


#[derive(Debug)]
pub struct Addr<'a> {
    pub host: &'a String,
    pub port: i32
}


// pub fn cread(addr: & str, port: i32) -> std::io::Result<String> {
//     let listener = TcpListener::bind(format!("{}:{}", addr, port))?;
//     let (mut stream, _) = listener.accept()?;
// 
//     let mut buf = [0; 1024];
//     let mut message = String::new();
// 
//     loop {
//         let bytes_read = stream.read(&mut buf)?;
//         let s = std::str::from_utf8(&buf[..bytes_read]).unwrap();
//         message.push_str(s);
// 
//         if bytes_read < buf.len() {
//             break;
//         }
//     }
// 
//     Ok(message)
// }

pub fn cwrite(addr: & str, port: i32, msg: & str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", addr, port))?;

    stream.write_all(msg.as_bytes())?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received response: {}", response);

    Ok(())
}


pub fn server(
    addr: &Addr, 
    mut handler: impl FnMut(&mut TcpStream) -> std::io::Result<()>
) -> std::io::Result<()> {
    //let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
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