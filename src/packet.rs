use std::string::FromUtf8Error;

pub const MIN_PACKET_SIZE: usize = 10;
/// The max packet size is 4096 not including the size field of 4 bytes.
pub const MAX_PACKET_SIZE: usize = 4096 + 4;

/// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Type
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    ResponseValue,
    ExecCommand,
    AuthResponse,
    Auth,
    /// Since both `ExecCommand` and `AuthResponse` are the value of 2, it depends on context what it is.
    ExecOrAuthResp,
    Raw(i32),
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => PacketType::ResponseValue,
            2 => PacketType::ExecOrAuthResp,
            3 => PacketType::Auth,
            x => PacketType::Raw(i32::from(x)),
        }
    }
}

impl From<i32> for PacketType {
    fn from(value: i32) -> Self {
        match value {
            0 => PacketType::ResponseValue,
            2 => PacketType::ExecOrAuthResp,
            3 => PacketType::Auth,
            x => PacketType::Raw(x),
        }
    }
}

impl From<&i32> for PacketType {
    fn from(value: &i32) -> Self {
        match value {
            0 => PacketType::ResponseValue,
            2 => PacketType::ExecOrAuthResp,
            3 => PacketType::Auth,
            x => PacketType::Raw(*x),
        }
    }
}

impl PartialEq for PacketType {
    fn eq(&self, other: &Self) -> bool {
        i32::from(self) == i32::from(other)
    }
}

impl Eq for PacketType {}

impl From<PacketType> for i32 {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
            PacketType::ExecOrAuthResp => 2,
            PacketType::Raw(x) => x,
        }
    }
}

impl From<&PacketType> for i32 {
    fn from(value: &PacketType) -> Self {
        match value {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
            PacketType::ExecOrAuthResp => 2,
            PacketType::Raw(x) => *x,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    ///The packet size field is a 32-bit little endian integer, representing the length of the request in bytes. Note that the packet size field itself is not included when determining the size of the packet, so the value of this field is always 4 less than the packet's actual length. The minimum possible value for packet size is 10:
    size: i32,
    /// The packet id field is a 32-bit little endian integer chosen by the client for each request. It may be set to any positive integer. When the server responds to the request, the response packet will have the same packet id as the original request (unless it is a failed SERVERDATA_AUTH_RESPONSE packet - see below.) It need not be unique, but if a unique packet id is assigned, it can be used to match incoming responses to their corresponding requests.
    id: i32,
    pkt_type: PacketType,
    body: String,
}

impl Packet {
    pub fn new(pkt_type: PacketType, body: String, id: i32) -> Packet {
        let size = (body.len() + MIN_PACKET_SIZE).try_into().unwrap();
        Packet::new_raw(pkt_type, body, size, id)
    }

    pub fn new_raw(pkt_type: PacketType, body: String, size: i32, id: i32) -> Packet {
        Packet {
            size,
            id,
            pkt_type,
            body,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_type(&self) -> PacketType {
        self.pkt_type
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PacketError {
    ParseError,
    InvalidPacketBody,
    UnexpectedID,
    UnexpectedType,
}

impl From<FromUtf8Error> for PacketError {
    fn from(_: FromUtf8Error) -> Self {
        PacketError::InvalidPacketBody
    }
}

impl From<Packet> for Vec<u8> {
    fn from(val: Packet) -> Self {
        // MIN_PACKET_SIZE + 2 for the 2 null bytes at the end.
        let mut output_vec = Vec::with_capacity(
            usize::try_from(val.size).expect("Invalid packet size") + MIN_PACKET_SIZE + 2,
        );
        output_vec.extend(val.size.to_le_bytes());
        output_vec.extend(val.id.to_le_bytes());
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
        let id = i32::from_le_bytes(value[4..8].try_into().expect("slice with incorrect length"));
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
        let pkt = Packet::new(PacketType::Auth, body, 1);
        assert_eq!(
            pkt.size,
            i32::try_from(expected_length + MIN_PACKET_SIZE).unwrap()
        );
    }

    #[test]
    fn test_auth_packet() {
        let pkt = Vec::from(Packet::new(PacketType::Auth, String::from("password"), 0));
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

        let expected_packet = Packet::new_raw(PacketType::ExecOrAuthResp, String::new(), 10, 0);

        assert_eq!(expected_packet, pkt);
    }
}
