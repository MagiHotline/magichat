use std::net::TcpListener;

/// Represents a Room's instance
///
/// Every room has its own alias and these will be automatically
/// connected to the first available connections.
#[derive(Debug)]
pub struct Room {
    pub alias: String,
    pub socket: TcpListener,
}
