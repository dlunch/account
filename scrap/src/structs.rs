use core::hash::{Hash, Hasher};

pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

pub mod internal {
    #[allow(clippy::module_inception)]
    pub mod internal {
        include!(concat!(env!("OUT_DIR"), "/proto.internal.rs"));
    }
}

impl Eq for internal::internal::Card {}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for internal::internal::Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.card_no.hash(state);
    }
}

pub use internal::internal::{Card, CardScrapRequest, CardTransaction};
