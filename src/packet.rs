use std::string::FromUtf8Error;

const MIN_PACKET_SIZE: usize = 10;

/// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Type
#[derive(Debug, Clone, Copy, Default)]
pub enum PacketType {
    #[default]
    ResponseValue,
    ExecCommand,
    AuthResponse,
    Auth,
    /// Since both (insert items) are the value of 2, it depends on context what it is.
    Unknown,
}

/// TODO: use better error type
impl TryFrom<u8> for PacketType {
    type Error = PacketError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::ResponseValue),
            2 => Ok(PacketType::Unknown),
            3 => Ok(PacketType::Auth),
            _ => Err(PacketError::ParseError),
        }
    }
}

/// TODO: use better error type
impl TryFrom<i32> for PacketType {
    type Error = PacketError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::ResponseValue),
            2 => Ok(PacketType::Unknown),
            3 => Ok(PacketType::Auth),
            _ => Err(PacketError::ParseError),
        }
    }
}

impl From<PacketType> for i32 {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
            PacketType::Unknown => 2,
        }
    }
}

impl From<PacketType> for u8 {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
            PacketType::Unknown => 2,
        }
    }
}

#[derive(Debug, Default)]
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
}

#[derive(Debug, Clone, Copy)]
pub enum PacketError {
    ParseError,
    InvalidPacketBody,
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
        output_vec.extend(i32::from(val.pkt_type).to_be_bytes());
        output_vec.extend(val.body.as_bytes());
        output_vec.extend([0u8, 0]);
        output_vec
    }
}

impl TryFrom<&[u8]> for Packet {
    type Error = PacketError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let body_end = value.len() - 2;
        let size = i32::from_le_bytes(value[0..3].try_into().expect("slice with incorrect length"));
        let id = i32::from_le_bytes(value[4..7].try_into().expect("slice with incorrect length"));
        let pkt_type = PacketType::try_from(i32::from_le_bytes(
            value[8..11]
                .try_into()
                .expect("slice with incorrect length"),
        ))?;
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
}
