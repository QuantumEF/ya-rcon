use std::io::{Error, Read, Write};

use crate::packet::{Packet, PacketError, PacketType, MAX_PACKET_SIZE};

// Question: Is this a bad way to convert errors?
/// The different errors that can arise from using the client.
/// I am not sure if the way I wrap the `SocketError` and `PacketError` is bad practice, but this is what I have for now.
#[derive(Debug)]
pub enum RCONError {
    /// This is an error return from whatever item was passed in as the "socket" option when creating a client: e.g. `TcpStream`
    SocketError(Error),
    /// This wraps any error returned from the `Packet` module. See `PacketError` for more info about the associated value contained in this type.
    PacketError(PacketError),
    /// The RCON server response indicated that the password was wrong.
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

/// The base RCON client. See the `RCONClient::new` function for info about the fields.
#[derive(Debug)]
pub struct RCONClient<T: Read + Write, I: Iterator<Item = u32>> {
    socket: T,
    incremental_id: I,
}

impl<T: Read + Write, I: Iterator<Item = u32>> RCONClient<T, I> {
    /// Creates a new instance of the `RCONClient`.
    ///
    /// # Arguments
    /// * `socket` - Any type that implements the `Read` and `Write` traits. This will usually be a `TcpStream` or similar, it could also be something like `websocket::client::sync::Client` (with some additional wrapping) if a game does things differently.
    /// * `id_generator` - Some iterator that yields u32, this is to fill the "ID" field of the packet. I reccomend simply using `0_u32..`
    /// * `password` - The password used to authenticate with the server.
    pub fn new(
        socket: T,
        id_generator: I,
        password: String,
    ) -> Result<RCONClient<T, I>, RCONError> {
        let mut client = RCONClient {
            socket,
            incremental_id: id_generator,
        };
        client.authenticate(password)?;
        Ok(client)
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
        let pkt = Vec::from(Packet::new(pkt_type, body, id)?);
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

    /// When `new()` is called this method will also be called, but it is exposed separatly in case it is desired. Not sure why it would be.
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

    /// Send the given command to the server and returns the response. This does not handle multipacket responses.
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
        let mut client = RCONClient::new(stream, 0.., password.to_string())?;
        client.authenticate(password.to_string())?;

        let reply = client.send_command("help".to_string())?;
        println!("RCON Server Reply: {reply}");
        Ok(())
    }
}
