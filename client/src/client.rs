use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

// Converting Client to UdpSocket to TcpStream
// The ability to create a Chat Room to a specified random address with an ALIAS (String)
// Client then goes on discover mode trying to see if it someone responds (the room that is always receiving)
#[derive(Debug)]
pub struct Client {
    pub name: String,
    pub stream: TcpStream, //TcpStream,
}

impl Client {
    pub fn new<A: ToSocketAddrs>(name: String, endpoint: A) -> std::io::Result<Self> {
        //let socket = UdpSocket::bind(endpoint)?;
        let stream = TcpStream::connect(endpoint)?;
        Ok(Self { name, stream })
    }

    /*
    pub fn connect<A: ToSocketAddrs>(&self, endpoint: A) -> std::io::Result<()> {
        self.stream.connect(endpoint)
    }
    */

    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self {
            name: self.name.clone(),
            stream: self.stream.try_clone()?,
        })
    }

    /// Send a message to the connected endpoint
    pub fn send(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.stream.write(data)
    }

    /// Receive a message from the connected endpoint
    pub fn recv(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(String::new(), "0.0.0.0:0").expect("Failed to bind UDP socket")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_binds_successfully() {
        let client = Client::new(String::new(), "0.0.0.0:0");
        assert!(client.is_ok());
    }

    #[test]
    fn client_has_local_addr() {
        let client = Client::new(String::new(), "0.0.0.0:0").unwrap();
        assert!(client.stream.local_addr().is_ok());
    }
}
