//! Contains the implementation for [`Packet`]

pub mod packet_error;
pub mod packet_id;
pub mod packet_type;

use packet_id::ID;

pub use crate::packet::{packet_error::PacketError, packet_type::PacketType};

/// The minimum packet size (in bytes) for an RCON packet.
/// Citation: <https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Size>
pub const MIN_PACKET_SIZE: usize = 10;
/// The max packet size is 4096 not including the size field of 4 bytes.
pub const MAX_PACKET_SIZE: usize = 4096 + 4;
/// The max size (in bytes) of a payload that can be sent.
pub const MAX_PAYLOAD_SIZE: usize = MAX_PACKET_SIZE - MIN_PACKET_SIZE;

/// Used to construct a RCON packet.
#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    /// The packet size field is a 32-bit little endian integer, representing the length of the request in bytes. Note that the packet size field itself is not included when determining the size of the packet, so the value of this field is always 4 less than the packet's actual length. The minimum possible value for packet size is 10:
    size: i32,
    /// The packet id field is a 32-bit little endian integer chosen by the client for each request. It may be set to any positive integer. When the server responds to the request, the response packet will have the same packet id as the original request (unless it is a failed SERVERDATA_AUTH_RESPONSE packet - see below.) It need not be unique, but if a unique packet id is assigned, it can be used to match incoming responses to their corresponding requests.
    id: ID,
    pkt_type: PacketType,
    body: String,
}

impl Packet {
    /// Creates a new packet with the given parameters, checks the body/payload length and calculates the size field of the packet.
    pub fn new(pkt_type: PacketType, body: String, id: ID) -> Result<Packet, PacketError> {
        if body.len() >= MAX_PAYLOAD_SIZE {
            return Err(PacketError::InvalidPayloadLength);
        }
        let size = (body.len() + MIN_PACKET_SIZE)
            .try_into()
            .expect("Earlier asertion should garentee this to pass");

        Ok(Packet::new_raw(pkt_type, body, size, id))
    }

    /// Creates a new packet with the given parameters but with no checks, Allows creating for an invalid packet.
    pub fn new_raw(pkt_type: PacketType, body: String, size: i32, id: ID) -> Packet {
        Packet {
            size,
            id,
            pkt_type,
            body,
        }
    }

    /// Gets the ID of the packet.
    pub fn get_id(&self) -> ID {
        self.id
    }

    /// Gets the packet body and performs a `clone()` to return it.
    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    /// Gets the [`PacketType`] of the packet.
    pub fn get_type(&self) -> PacketType {
        self.pkt_type
    }
}

impl From<Packet> for Vec<u8> {
    fn from(val: Packet) -> Self {
        // MIN_PACKET_SIZE + 2 for the 2 null bytes at the end.
        let mut output_vec = Vec::with_capacity(
            usize::try_from(val.size).expect("Invalid packet size") + MIN_PACKET_SIZE + 2,
        );
        output_vec.extend(val.size.to_le_bytes());
        output_vec.extend(i32::from(val.id).to_le_bytes());
        output_vec.extend(i32::from(val.pkt_type).to_le_bytes());
        output_vec.extend(val.body.as_bytes());
        output_vec.extend([0u8, 0]);
        output_vec
    }
}

impl TryFrom<&[u8]> for Packet {
    type Error = PacketError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let body_end = value.len() - 2;
        let size = i32::from_le_bytes(value[0..4].try_into().expect("slice with incorrect length"));
        let id =
            i32::from_le_bytes(value[4..8].try_into().expect("slice with incorrect length")).into();
        let pkt_type = PacketType::from(i32::from_le_bytes(
            value[8..12]
                .try_into()
                .expect("slice with incorrect length"),
        ));
        let body = String::from_utf8(value[12..body_end].to_vec())?;

        Ok(Packet::new_raw(pkt_type, body, size, id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_packet_size() {
        let body = String::from("test");
        let expected_length = body.len();
        let pkt = Packet::new(PacketType::Auth, body, ID::from(1)).unwrap();
        assert_eq!(
            pkt.size,
            i32::try_from(expected_length + MIN_PACKET_SIZE).unwrap()
        );
    }

    #[test]
    fn test_auth_packet() {
        let pkt = Vec::from(
            Packet::new(PacketType::Auth, String::from("password"), ID::from(0)).unwrap(),
        );
        // Expected output generated from packet capture using https://github.com/gorcon/rcon-cli
        let expected_bytes = [
            0x12u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x70, 0x61,
            0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x00, 0x00,
        ];

        assert_eq!(&expected_bytes[..], &pkt[..]);
    }

    #[test]
    fn test_parse_auth_reply() {
        let raw_data = [
            0x0au8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let pkt = Packet::try_from(&raw_data[..]).unwrap();

        let expected_packet =
            Packet::new_raw(PacketType::ExecOrAuthResp, String::new(), 10, ID::from(0));

        assert_eq!(expected_packet, pkt);
    }
}
