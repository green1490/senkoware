use std::{io::Write, net::TcpStream};
use crate::entity::connection::Connection;

// pub fn tpc<'a>(connection: &Connection) -> Result<TcpStream,std::io::Error> {
//     let mut stream = TcpStream::connect(format!("{}:{}" , connection.ip, connection.port))?
// }

pub struct Tcp {
    connection: Connection,
    stream: TcpStream
}

impl Tcp {
    pub fn new(connection: Connection) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(format!("{}:{}" , connection.ip, connection.port))?;
        Ok(
            Tcp {
            connection,
            stream
        })
    }

    pub fn write(&mut self, text:&str) {
        let _ = &self.stream.write(text.as_bytes());
    }
}