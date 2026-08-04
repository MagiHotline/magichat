use std::net::{ToSocketAddrs, UdpSocket};

#[derive(Debug)]
pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub fn new() -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self { socket })
    }

    pub fn connect<A: ToSocketAddrs>(&self, endpoint: A) -> std::io::Result<()> {
        self.socket.connect(endpoint)
    }

    /// Send a message to the connected endpoint
    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        self.socket.send(data)
    }

    /// Receive a message from the connected endpoint
    pub fn recv(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.socket.recv(buf)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new().expect("Failed to bind UDP socket")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_binds_successfully() {
        let client = Client::new();
        assert!(client.is_ok());
    }

    #[test]
    fn client_has_local_addr() {
        let client = Client::new().unwrap();
        assert!(client.socket.local_addr().is_ok());
    }
}
