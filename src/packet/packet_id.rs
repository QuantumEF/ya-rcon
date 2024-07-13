/// This struct is about explicitly stating how the ID is handled see `from_wrapping` for more info
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct ID(i32);

impl ID {
    /// Valid IDs are positive integers, but it can take on negative values when the server wants to indicate an error.
    /// This takes u32 to ensure the value is possitive and wraps it simply by using bitmasking
    pub fn from_wrapping(id: u32) -> ID {
        ID((id & 0xEFFFFFFF) as i32)
    }

    pub fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl From<ID> for i32 {
    fn from(value: ID) -> Self {
        value.0
    }
}

impl From<i32> for ID {
    fn from(value: i32) -> Self {
        ID(value)
    }
}
