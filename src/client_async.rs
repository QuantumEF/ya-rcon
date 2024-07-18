//! Contains the implementation for [`RCONClient`]
#[cfg(feature = "async-net")]
use futures::{AsyncReadExt, AsyncWriteExt};
use std::io::{Error, ErrorKind};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::packet::{packet_id::ID, Packet, PacketError, PacketType, MAX_PACKET_SIZE};

/// The base AsyncRCON client. See the [`AsyncRCONClient::new()`] function for info about the fields.
#[derive(Debug)]
pub struct AsyncRCONClient<T: AsyncReadExt + AsyncWriteExt, I: Iterator<Item = ID>> {
    socket: T,
    incremental_id: I,
}

impl<T: AsyncReadExt + AsyncWriteExt + Unpin, I: Iterator<Item = ID>> AsyncRCONClient<T, I> {
    /// Creates a new instance of the [`AsyncRCONClient`].
    ///
    /// # Arguments
    /// * `socket` - Any type that implements the [`AsyncReadExt`] and [`AsyncWriteExt`] traits.
    /// * `id_generator` - Some iterator that yields [`ID`], this is to fill the "ID" field of the packet.
    /// * `password` - The password used to authenticate with the server.
    pub async fn new(
        socket: T,
        id_generator: I,
        password: String,
    ) -> Result<AsyncRCONClient<T, I>, Error> {
        let mut client = AsyncRCONClient {
            socket,
            incremental_id: id_generator,
        };
        client.authenticate(password).await?;
        Ok(client)
    }

    fn next_id(&mut self) -> ID {
        self.incremental_id
            .next()
            .expect("Iterator should have been infinate, how should I handle?")
    }

    async fn send_packet(&mut self, pkt_type: PacketType, body: String) -> Result<ID, Error> {
        let id = self.next_id();
        let pkt = Vec::from(Packet::new(pkt_type, body, id)?);
        self.socket.write_all(&pkt).await?;
        Ok(id)
    }

    async fn recv_packet_unchecked(&mut self) -> Result<Packet, Error> {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        let packet_len = self.socket.read(&mut buf).await?;
        // Question about buf[..]
        let packet = Packet::try_from(&buf[..packet_len])?;
        Ok(packet)
    }

    async fn recv_packet(
        &mut self,
        expected_type: PacketType,
        expected_id: ID,
    ) -> Result<String, Error> {
        let packet = self.recv_packet_unchecked().await?;
        if packet.get_id() != expected_id {
            Err(PacketError::UnexpectedID)?;
        }
        if packet.get_type() != expected_type {
            Err(PacketError::UnexpectedType)?;
        }
        Ok(packet.get_body())
    }

    /// When [`AsyncRCONClient::new()`] is called this method will also be called, but it is exposed separatly in case it is desired. Not sure why it would be.
    pub async fn authenticate(&mut self, password: String) -> Result<(), Error> {
        let used_id = self.send_packet(PacketType::Auth, password).await?;
        self.wait_authentication(used_id).await
    }

    async fn wait_authentication(&mut self, expected_id: ID) -> Result<(), Error> {
        let packet = self.recv_packet_unchecked().await?;

        if packet.get_type() != PacketType::AuthResponse {
            return Err(PacketError::UnexpectedType.into());
        }

        let packet_id = packet.get_id();
        if packet_id == (-1).into() {
            Err(Error::new(
                ErrorKind::PermissionDenied,
                "Authentication with the RCONserver failed.",
            ))
        } else if expected_id == packet_id {
            Ok(())
        } else {
            Err(PacketError::UnexpectedID.into())
        }
    }

    /// Send the given command to the server and returns the response. This does not handle multipacket responses.
    pub async fn send_command(&mut self, cmd: String) -> Result<String, Error> {
        let used_id = self.send_packet(PacketType::ExecCommand, cmd).await?;
        self.recv_packet(PacketType::ResponseValue, used_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimpleIDGenerator;

    #[tokio_macros::test]
    #[ignore = "Requires RCON Server"]
    async fn basic_rcon_client_test() {
        // Look at the example_rcon_server.txt file as an example for your rcon_server.txt file.
        // Open to alternate suggestions.
        let (address, password) = include!("../rcon_server.txt");
        let stream = async_net::TcpStream::connect(address).await.unwrap();
        let mut client =
            AsyncRCONClient::new(stream, SimpleIDGenerator::new(), password.to_string())
                .await
                .unwrap();
        client.authenticate(password.to_string()).await.unwrap();

        let reply = client.send_command("help".to_string()).await.unwrap();
        println!("RCON Server Reply: {reply}");
    }
}
