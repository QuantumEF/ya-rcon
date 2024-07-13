/// See <https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Type> for info about the types.
/// I have done this a little weirdly since the underlying value changed depending on context.
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum PacketType {
    ResponseValue,
    ExecCommand,
    AuthResponse,
    Auth,
    /// Since both `ExecCommand` and `AuthResponse` are the value of 2, it depends on context what it is. Can probably just replace this with `PacketType::Raw(2)`
    ExecOrAuthResp,
    /// A raw version of the packet type used since `PacketType::AuthResponse` and `PacketType::ExecCommand` have the same value.
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
