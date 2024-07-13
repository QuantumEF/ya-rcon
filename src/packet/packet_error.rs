use std::{io::Error, string::FromUtf8Error};

#[derive(Debug, Clone, Copy)]
pub enum PacketError {
    ParseError,
    InvalidPacketBody,
    InvalidPayloadLength,
    UnexpectedID,
    UnexpectedType,
}

impl From<FromUtf8Error> for PacketError {
    fn from(_: FromUtf8Error) -> Self {
        PacketError::InvalidPacketBody
    }
}

impl From<PacketError> for Error {
    fn from(value: PacketError) -> Self {
        Error::other(format!("{:?}", value))
    }
}
