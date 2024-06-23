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

    fn send_authentication(&mut self, password: String) -> Result<(), std::io::Error> {
        let packet = Vec::from(Packet::new(
            PacketType::Auth,
            password,
            self.incremental_id.get(),
        ));
        self.socket.write_all(&packet)
    }

    fn wait_authentication(&mut self) -> Result<(), RCONError> {
        let mut buf = [0u8; 10];
        let _ = self.socket.read(&mut buf)?;
        // Question about buf[..]
        let packet = Packet::try_from(&buf[..])?;
        if packet.get_id() == -1 {
            return Err(RCONError::AuthenticationFailed);
        }
        Ok(())
    }

    pub fn authenticate(&mut self, password: String) -> Result<(), RCONError> {
        self.send_authentication(password)?;
        self.wait_authentication()
    }

    pub fn send_command(&mut self, cmd: String) -> Result<(), Error> {
        let pkt = Vec::from(Packet::new(
            PacketType::ExecCommand,
            cmd,
            self.incremental_id.get(),
        ));
        self.socket.write_all(&pkt)
    }

    pub fn get_response(&mut self) -> Result<String, RCONError> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        self.socket.read_exact(&mut buf)?;
        let pkt = Packet::try_from(&buf[..])?;
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
        println!("{:?}", client.send_command("list".to_string()));

        Ok(())
    }
}
