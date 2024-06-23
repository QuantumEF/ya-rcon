use std::io::{Error, Read, Write};

use crate::packet::{Packet, PacketError, PacketType};

// TODO: Set to actual value.
const MAX_PACKET_SIZE: usize = 4096 + 64;

// Question: Is this a bad way to convert errors?
#[derive(Debug)]
pub enum RCONError {
    SocketError(Error),
    PacketError(PacketError),
    AuthenticationFailed,
}

impl From<Error> for RCONError {
    fn from(value: Error) -> Self {
        RCONError::SocketError(value)
    }
}

impl From<PacketError> for RCONError {
    fn from(value: PacketError) -> Self {
        RCONError::PacketError(value)
    }
}

#[derive(Debug)]
struct IncrementingID(i32);
impl IncrementingID {
    fn new() -> IncrementingID {
        IncrementingID(1)
    }

    fn get(&mut self) -> i32 {
        let id = self.0;
        self.0 += 1;
        id
    }
}

#[derive(Debug)]
pub struct RCONClient<T: Read + Write> {
    socket: T,
    incremental_id: IncrementingID,
}

impl<T: Read + Write> RCONClient<T> {
    pub fn new(socket: T) -> RCONClient<T> {
        RCONClient {
            socket,
            incremental_id: IncrementingID::new(),
        }
    }

    fn send_authentication(&mut self, password: String) -> Result<i32, std::io::Error> {
        let used_id = self.incremental_id.get();
        let packet = Vec::from(Packet::new(PacketType::Auth, password, used_id));
        self.socket.write_all(&packet)?;
        Ok(used_id)
    }

    fn wait_authentication(&mut self, expected_id: i32) -> Result<(), RCONError> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        let _ = self.socket.read(&mut buf)?;
        // Question about buf[..]
        let packet = Packet::try_from(&buf[..])?;

        if packet.get_type() != PacketType::AuthResponse {
            return Err(RCONError::PacketError(PacketError::UnexpectedType));
        }

        let packet_id = packet.get_id();
        if packet_id == -1 {
            Err(RCONError::AuthenticationFailed)
        } else if expected_id == packet_id {
            Ok(())
        } else {
            Err(RCONError::PacketError(PacketError::UnexpectedID))
        }
    }

    pub fn authenticate(&mut self, password: String) -> Result<(), RCONError> {
        let used_id = self.send_authentication(password)?;
        self.wait_authentication(used_id)
    }

    pub fn send_command(&mut self, cmd: String) -> Result<(), Error> {
        let pkt = Vec::from(Packet::new(
            PacketType::ExecCommand,
            cmd,
            self.incremental_id.get(),
        ));
        self.socket.write_all(&pkt)
    }

    pub fn get_packet(&mut self) -> Result<Packet, RCONError> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        self.socket.read_exact(&mut buf)?;
        let pkt = Packet::try_from(&buf[..])?;
        Ok(pkt)
    }

    pub fn get_response(&mut self) -> Result<String, RCONError> {
        let pkt = self.get_packet()?;
        Ok(pkt.get_body())
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpStream;

    use super::RCONClient;

    #[test]
    #[ignore = "Requires RCON Server"]
    fn basic_rcon_client_test() -> std::io::Result<()> {
        let stream = TcpStream::connect("127.0.0.1:27016")?;
        let mut client = RCONClient::new(stream);
        println!("{:?}", client.send_authentication("password".to_string()));
        println!("{:?}", client.get_packet());
        println!("{:?}", client.send_command("list".to_string()));

        Ok(())
    }
}
