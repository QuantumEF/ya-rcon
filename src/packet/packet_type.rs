//! Contains the implementation for [`PacketType`]

/// See <https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Type> for more info about the types.
/// The documentation for most of the types was taken directly from there.
/// I have done this a little weirdly since the underlying value changed depending on context.
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    /// [SERVERDATA_RESPONSE_VALUE](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_RESPONSE_VALUE): Server response to an [`PacketType::ExecCommand`]
    ResponseValue,
    /// [SERVERDATA_EXECCOMMAND](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_EXECCOMMAND): Packet containing the command sent from the client to the server.
    ExecCommand,
    /// [SERVERDATA_AUTH_RESPONSE](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_AUTH_RESPONSE): Part of the server response indicating if the authentication was sucessfull.
    AuthResponse,
    /// [SERVERDATA_AUTH](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_AUTH): Usually the first packet sent from client to server. Contains the password.
    Auth,
    /// Since both [`PacketType::ExecCommand`] and [`PacketType::AuthResponse`] are the value of 2, it depends on context what it is. Can probably just replace this with `PacketType::Raw(2)`
    ExecOrAuthResp,
    /// A raw version of the packet type used since [`PacketType::AuthResponse`] and [`PacketType::ExecCommand`] have the same value.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_type_equality_tests() {
        assert_eq!(PacketType::AuthResponse, PacketType::ExecCommand);
        assert_eq!(PacketType::AuthResponse, PacketType::ExecOrAuthResp);
        assert_eq!(PacketType::ExecCommand, PacketType::ExecOrAuthResp);
        assert_eq!(PacketType::Auth, PacketType::Raw(3));
    }
}
