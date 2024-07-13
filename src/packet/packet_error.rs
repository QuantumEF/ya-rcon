use std::{io::Error, string::FromUtf8Error};

/// The errors that can happen when working with a [`crate::Packet`]
#[derive(Debug, Clone, Copy)]
pub enum PacketError {
    /// Turns out I have not actually used this anywhere.
    ParseError,
    /// Used if the packet is not valid ASCII/UTF8
    InvalidPacketBody,
    /// The String payload exceded the [`crate::packet::MAX_PAYLOAD_SIZE`]
    InvalidPayloadLength,
    /// I am using this primarily in [`crate::RCONClient`]. Maybe remove it from here and put elsewhere?
    /// Used when comparing the ID of the recieved packed to the ID used when sending a packet.
    UnexpectedID,
    /// I am using this primarily in [`crate::RCONClient`]. Maybe remove it from here and put elsewhere?
    /// When sending packets, it is expected the replys be of a certain type. This error is used for when the reply type is not the expected one.
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
