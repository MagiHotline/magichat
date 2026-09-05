use std::{
    io::{self},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
};

/// Represents a Room's instance
///
/// Every room has its own alias and these will be automatically
/// connected to the first available connections.
#[derive(Debug)]
pub struct Room {
    pub alias: String,
    pub socket: Option<TcpListener>,
}

impl Room {
    pub fn create(alias: String) -> io::Result<Room> {
        // Discover and find an available address between 192.168.1.1 to 192.168.1.254
        let mut socket: Option<TcpListener> = None;
        for i in 1..254 {
            let try_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, i)), 1234);
            if let Ok(tcp_listener) = TcpListener::bind(try_socket) {
                socket = Some(tcp_listener);
                break;
            };
        }

        Ok(Self { alias, socket })
    }

    pub fn create_test(alias: String) -> io::Result<Room> {
        // Discover and find an available address
        let tcp_listener = TcpListener::bind("127.0.0.1:8000")?;

        Ok(Self {
            alias,
            socket: Some(tcp_listener),
        })
    }
}
