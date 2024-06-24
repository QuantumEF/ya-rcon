use std::string::FromUtf8Error;

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
