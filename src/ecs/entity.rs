use std::ops::Deref;

use crate::mem::gen_bit_set::GenBit;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity(pub(super) GenBit);

impl Deref for Entity {
    type Target = GenBit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
