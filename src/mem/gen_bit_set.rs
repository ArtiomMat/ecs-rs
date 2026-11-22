// use bitvec::{bitvec, vec::BitVec};
use crate::mem::bit_set::BitSet;
use crate::mem::gen_bit_set::Error::OutOfBounds;
use std::num::NonZero;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Full")]
    Full,
    #[error("The index was out of bounds {margin}")]
    OutOfBounds { margin: usize },
    #[error("Bit at index {index} was already released with generation diff {generation_diff}")]
    BitReleased { index: usize, generation_diff: i32 },
    #[error("Bit has been poisoned")]
    BitPoisoned,
}

pub struct GenBitSet {
    bits: BitSet,
    generations: Vec<u32>
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GenBit {
    index_plus_one: Option<NonZero<u32>>,
    generation: u32,
}

impl GenBit {
    fn new(index: usize, generation: u32) -> Option<Self> {
        let Ok(index) = (index + 1).try_into() else {
            return None;
        };
        let Some(index) = NonZero::<u32>::new(index) else {
            return None;
        };

        Some(Self {
            index_plus_one: Some(index),
            generation: generation,
        })
    }

    pub fn poison(&mut self) {
        self.index_plus_one = None;
    }

    fn index(&self) -> Option<usize> {
        self.index_plus_one.map(|x| x.get() as usize - 1)
    }
}

impl GenBitSet {
    pub fn new(size: usize) -> Self {
        let bits = BitSet::new(size, false);
        let generations = vec![0; size];

        Self {
            bits,
            generations
        }
    }

    pub fn resize(&mut self, size: usize) {
        self.bits.resize(size, false);
        self.generations.resize(size, 0);
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn contains(&self, bit: GenBit) -> bool {
        self.downgrade(bit).is_ok()
    }

    pub fn acquire(&mut self) -> Option<GenBit> {
        let Some(index) = self.bits.first_zero() else {
            return None;
        };
        self.bits.set(index, true);

        let generation = self.generations[index];

        GenBit::new(index, generation)
    }

    pub fn release(&mut self, bit: GenBit) -> Result<(), Error> {
        let index = self.downgrade(bit)?;
        self.bits.set(index, false);
        self.generations[index] += 1;
        Ok(())
    }

    pub fn downgrade(&self, bit: GenBit) -> Result<usize, Error> {
        if let Some(index) = bit.index() {
            let is_index_set = self.bits.get(index).unwrap_or(false);
            let generation_diff = self.generations[index] as i64 - bit.generation as i64;
            if is_index_set && generation_diff == 0 {
                Ok(index)
            } else {
                Err(Error::BitReleased {
                    index,
                    generation_diff: generation_diff as i32,
                })
            }
        } else {
            Err(Error::BitPoisoned)
        }
    }

    pub fn upgrade(&self, index: usize) -> Result<GenBit, Error> {
        if let Some(index) = self.bits.get(index).map(|x| x as usize) {
            Ok(GenBit::new(index.into(), self.generations[index]).unwrap())
        } else {
            Err(OutOfBounds {
                margin: index - self.bits.len(),
            })
        }
    }

    pub fn downgrade_or_poison(&self, bit: &mut GenBit) -> Result<usize, Error> {
        self.downgrade(*bit).inspect_err(|_| bit.poison())
    }

    pub fn release_or_poison(&mut self, bit: &mut GenBit) -> Result<(), Error> {
        let index = self.downgrade_or_poison(bit)?;
        self.bits.set(index, false);
        self.generations[index] += 1;
        Ok(())
    }
}
