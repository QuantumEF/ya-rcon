use crate::packet::packet_id::ID;
struct SimpleIDGenerator(u32);

impl Iterator for SimpleIDGenerator {
    type Item = ID;
    fn next(&mut self) -> Option<Self::Item> {
        let id = ID::from_wrapping(self.0);
        self.0 = self.0.wrapping_add(1);
        Some(id)
    }
}
