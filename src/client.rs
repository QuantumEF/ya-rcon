use std::io::{Error, Read, Write};

use crate::packet::{Packet, PacketError, PacketType, MAX_PACKET_SIZE};

// Question: Is this a bad way to convert errors?
#[derive(Debug)]
pub enum RCONError {
    SocketError(Error),
    PacketError(PacketError),
    AuthenticationFailed,
}

impl From<RCONError> for Error {
    fn from(value: RCONError) -> Self {
        Error::other(format!("{:?}", value))
    }
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

    fn send_packet(&mut self, pkt_type: PacketType, body: String) -> Result<i32, Error> {
        let id = self.next_id();
        let pkt = Vec::from(Packet::new(pkt_type, body, id));
        self.socket.write_all(&pkt)?;
        Ok(id)
    }

    fn recv_packet_unchecked(&mut self) -> Result<Packet, Error> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        let packet_len = self.socket.read(&mut buf)?;
        // Question about buf[..]
        let packet = Packet::try_from(&buf[..packet_len])?;
        Ok(packet)
    }

    fn recv_packet(
        &mut self,
        expected_type: PacketType,
        expected_id: i32,
    ) -> Result<String, Error> {
        let packet = self.recv_packet_unchecked()?;
        if packet.get_id() != expected_id {
            Err(PacketError::UnexpectedID)?;
        }
        if packet.get_type() != expected_type {
            Err(PacketError::UnexpectedType)?;
        }
        Ok(packet.get_body())
    }

    pub fn authenticate(&mut self, password: String) -> Result<(), RCONError> {
        let used_id = self.send_packet(PacketType::Auth, password)?;
        self.wait_authentication(used_id)
    }

    fn wait_authentication(&mut self, expected_id: i32) -> Result<(), RCONError> {
        let packet = self.recv_packet_unchecked()?;

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

    pub fn send_command(&mut self, cmd: String) -> Result<String, Error> {
        let used_id = self.send_packet(PacketType::ExecCommand, cmd)?;
        self.recv_packet(PacketType::ResponseValue, used_id)
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Error, net::TcpStream};

    use super::*;

    #[test]
    #[ignore = "Requires RCON Server"]
    fn basic_rcon_client_test() -> Result<(), Error> {
        // Look at the example_rcon_server.txt file as an example for your rcon_server.txt file.
        // Open to alternate suggestions.
        let (address, password) = include!("../rcon_server.txt");
        let stream = TcpStream::connect(address)?;
        let mut client = RCONClient::new(stream, 0..);
        client.authenticate(password.to_string())?;

        let reply = client.send_command("help".to_string())?;
        println!("RCON Server Reply: {reply}");
        Ok(())
    }
}
