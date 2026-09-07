use std::{
    io::{Error, ErrorKind, Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::client::ClientState::{Connected, Disconnected};

#[derive(Debug)]
pub enum ClientState {
    Disconnected,
    Connected(TcpStream),
}

// Converting Client to UdpSocket to TcpStream
// The ability to create a Chat Room to a specified random address with an ALIAS (String)
// Client then goes on discover mode trying to see if it someone responds (the room that is always receiving)
#[derive(Debug)]
pub struct Client {
    pub name: String,
    pub state: ClientState,
}

impl Client {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: Disconnected,
        }
    }

    /// You can create a new client only if the endpoint is already receiving
    pub fn connect<A: ToSocketAddrs>(name: String, endpoint: A) -> std::io::Result<Self> {
        let stream = TcpStream::connect(endpoint)?;
        Ok(Self {
            name,
            state: Connected(stream),
        })
    }

    pub fn try_clone(&self) -> std::io::Result<Self> {
        let cloned_state = match &self.state {
            Disconnected => ClientState::Disconnected,
            Connected(tcp_stream) => ClientState::Connected(tcp_stream.try_clone()?),
        };

        Ok(Self {
            name: self.name.clone(),
            state: cloned_state,
        })
    }

    /// Send a message to the connected endpoint
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.state {
            Disconnected => Err(Error::new(
                ErrorKind::NotConnected,
                "Socket is not connected",
            )),
            Connected(tcp_stream) => tcp_stream.write(buf),
        }
    }

    /// Receive a message from the connected endpoint
    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.state {
            Disconnected => Err(Error::new(
                ErrorKind::NotConnected,
                "Socket is not connected",
            )),
            Connected(tcp_stream) => tcp_stream.read(buf),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            name: String::new(),
            state: Disconnected,
        }
    }
}
