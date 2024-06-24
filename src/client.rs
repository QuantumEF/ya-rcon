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
pub struct RCONClient<T: Read + Write, I: Iterator<Item = u32>> {
    socket: T,
    incremental_id: I,
}

impl<T: Read + Write, I: Iterator<Item = u32>> RCONClient<T, I> {
    pub fn new(socket: T, id_generator: I) -> RCONClient<T, I> {
        RCONClient {
            socket,
            incremental_id: id_generator,
        }
    }

    fn next_id(&mut self) -> i32 {
        i32::try_from(
            self.incremental_id
                .next()
                .expect("Iterator should have been infinate, how should I handle?")
                & 0xEFFFFFFF,
        )
        .expect("Bit masking did not work")
    }

    fn send_authentication(&mut self, password: String) -> Result<i32, std::io::Error> {
        let used_id = self.next_id();
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
        let pkt = Vec::from(Packet::new(PacketType::ExecCommand, cmd, self.next_id()));
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
        let mut client = RCONClient::new(stream, 0..);
        println!("{:?}", client.send_authentication("password".to_string()));
        println!("{:?}", client.send_command("list".to_string()));
        println!("{:?}", client.get_packet());
        Ok(())
    }
}
