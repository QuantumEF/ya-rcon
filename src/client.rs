use std::io::{Error, Read, Write};

use crate::packet::{Packet, PacketError, PacketType};

// TODO: Set to actual value.
const MAX_PACKET_SIZE: usize = 4096 + 64;

// Question: Is this a bad way to convert errors?
pub enum AuthenticationError {
    SocketError(Error),
    PacketError(PacketError),
    AuthenticationFailed,
}

impl From<Error> for AuthenticationError {
    fn from(value: Error) -> Self {
        AuthenticationError::SocketError(value)
    }
}

impl From<PacketError> for AuthenticationError {
    fn from(value: PacketError) -> Self {
        AuthenticationError::PacketError(value)
    }
}

pub struct RCONClient<T: Read + Write> {
    socket: T,
}

impl<T: Read + Write> RCONClient<T> {
    pub fn new(socket: T) -> RCONClient<T> {
        RCONClient { socket }
    }

    fn send_authentication(&mut self, password: String) -> Result<(), std::io::Error> {
        let packet = Vec::from(Packet::new(PacketType::Auth, password));
        self.socket.write_all(&packet)
    }

    fn wait_authentication(&mut self) -> Result<(), AuthenticationError> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        let _ = self.socket.read(&mut buf)?;
        // Question about buf[..]
        let packet = Packet::try_from(&buf[..])?;
        if packet.get_id() == -1 {
            return Err(AuthenticationError::AuthenticationFailed);
        }
        Ok(())
    }

    pub fn authenticate(&mut self, password: String) -> Result<(), AuthenticationError> {
        self.send_authentication(password)?;
        self.wait_authentication()
    }
}
