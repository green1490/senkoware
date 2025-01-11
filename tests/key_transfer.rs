use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

#[test]
fn tcp_write() -> Result<(), std::io::Error> {
    const IP:&str = "127.0.0.1:4444";
    
    let listener = TcpListener::bind(IP);
    let mut buffer:Vec<u8> = vec![];
    let mut streamer = TcpStream::connect(IP)?;
    let _ = streamer.write("Transfering the key...".as_bytes());

    Ok(())
}