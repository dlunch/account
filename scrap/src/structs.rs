use core::hash::{Hash, Hasher};

include!(concat!(env!("OUT_DIR"), "/proto.internal.rs"));

impl Eq for Card {}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.card_no.hash(state);
    }
}
