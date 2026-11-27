use std::num::NonZero;

use crate::mem::BitSet;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bit at index {index} was already released with generation diff {generation_diff}")]
    BitReleased { index: u32, generation_diff: i32 },
}

/// Manages a bit-set with generations tied to each bit.
///
/// Useful when you need handles.
pub struct GenBitSet {
    bits: BitSet,
    generations: Vec<u32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GenBit {
    index: u32,
    generation: u32,
}

impl GenBit {
    /// Returns the index without the generation.
    pub fn index(&self) -> u32 {
        self.index
    }
}

impl GenBitSet {
    pub fn new(size: usize) -> Self {
        let bits = BitSet::with_capacity(size);
        let generations = vec![0; size];

        Self { bits, generations }
    }

    pub fn resize(&mut self, size: usize) {
        self.bits.grow(size);
        self.generations.resize(size, 0);
    }

    pub fn capacity(&self) -> usize {
        self.bits.capacity()
    }

    pub fn contains(&self, bit: GenBit) -> bool {
        self.downgrade(bit).is_ok()
    }

    pub fn acquire(&mut self) -> Option<GenBit> {
        let Some(index) = self.bits.zeroes().next() else {
            return None;
        };
        self.bits.set(index, true);

        let generation = self.generations[index];

        Some(GenBit {
            index: index as u32,
            generation,
        })
    }

    pub fn release(&mut self, bit: GenBit) -> Result<(), Error> {
        let index = self.downgrade(bit)? as usize;
        self.bits.set(index, false);
        self.generations[index] += 1;
        Ok(())
    }

    pub fn downgrade(&self, bit: GenBit) -> Result<u32, Error> {
        let index = bit.index;
        let contains_bit = self.bits.contains(index as usize);
        let generation_diff = self.generations[index as usize] as i64 - bit.generation as i64;
        if contains_bit && generation_diff == 0 {
            Ok(index)
        } else {
            Err(Error::BitReleased {
                index,
                generation_diff: generation_diff as i32,
            })
        }
    }

    /// Returns the full bit(with generation) out of index if it is still acquired, otherwise `None`.
    ///
    /// Note that there is no way to verify what generation you are expecting, this function
    /// exists precisely to recover the full bit.
    pub fn upgrade(&self, index: usize) -> Option<GenBit> {
        if self.bits.contains(index) {
            Some(GenBit {
                index: index as u32,
                generation: self.generations[index],
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: usize = 8;

    fn set() -> GenBitSet {
        GenBitSet::new(SIZE)
    }

    #[test]
    fn new() {
        let set = GenBitSet::new(SIZE);

        assert_eq!(set.capacity(), SIZE);
        assert_eq!(set.bits.ones().count(), 0, "no bits should be set");
        assert_eq!(set.generations.len(), SIZE);
        assert!(
            set.generations.iter().all(|&g| g == 0),
            "generations should start at 0"
        );
    }

    #[test]
    fn acquire() {
        let mut set = set();

        let bit = set.acquire().expect("set has free space");
        assert!(set.contains(bit));

        // A second acquire gives a different, also-contained bit.
        let other = set.acquire().expect("set has free space");
        assert_ne!(bit, other);
        assert!(set.contains(bit));
        assert!(set.contains(other));
    }

    #[test]
    fn acquire_release() {
        let mut set = set();

        let bit = set.acquire().unwrap();
        let index = set.downgrade(bit).unwrap() as usize;
        assert_eq!(set.generations[index], 0);

        set.release(bit).expect("releasing a live bit works");

        assert!(!set.contains(bit), "released bit must not be contained");
        assert!(!set.bits.contains(index), "underlying bit must be cleared");
        assert_eq!(set.generations[index], 1, "generation must be incremented");
    }

    #[test]
    fn resize() {
        let mut set = set();
        let bit = set.acquire().unwrap();

        let new_size = SIZE * 2;
        set.resize(new_size);

        assert_eq!(set.capacity(), new_size);
        assert_eq!(set.generations.len(), new_size);
        assert!(set.generations[SIZE..].iter().all(|&g| g == 0));

        // Existing handles remain valid.
        assert!(set.contains(bit));

        // The new capacity is actually usable: we can acquire up to `new_size` bits.
        let mut bits = vec![bit];
        while let Some(b) = set.acquire() {
            bits.push(b);
        }
        assert_eq!(bits.len(), new_size);
        for b in bits {
            assert!(set.contains(b));
            assert!(set.downgrade(b).unwrap() < new_size as u32);
        }
    }

    #[test]
    fn downgrade() {
        let mut set = set();
        let bit = set.acquire().unwrap();

        let index = set.downgrade(bit).expect("live bit downgrades");
        assert!(index < set.capacity() as u32);
        assert!(set.bits.contains(index as usize));

        // Downgrading is non-destructive and repeatable.
        assert_eq!(set.downgrade(bit).unwrap(), index);
        assert!(set.contains(bit));
    }

    #[test]
    fn downgrade_released() {
        let mut set = set();
        let bit = set.acquire().unwrap();
        let index = set.downgrade(bit).unwrap();
        set.release(bit).unwrap();

        match set.downgrade(bit) {
            Err(Error::BitReleased {
                index: i,
                generation_diff,
            }) => {
                assert_eq!(i, index);
                assert_eq!(generation_diff, 1);
            }
            other => panic!("expected BitReleased, got {other:?}"),
        }
    }

    #[test]
    fn upgrade() {
        let mut set = set();
        let bit = set.acquire().unwrap();
        let index = set.downgrade(bit).unwrap() as usize;

        let upgraded = set.upgrade(index).expect("live index upgrades");
        assert_eq!(upgraded, bit);
        assert!(set.contains(upgraded));
    }

    #[test]
    fn upgrade_released() {
        let mut set = set();
        let bit = set.acquire().unwrap();
        let index = set.downgrade(bit).unwrap() as usize;
        set.release(bit).unwrap();

        assert!(set.upgrade(index).is_none());
    }

    // ---------- Proposed extras ----------

    #[test]
    fn acquire_when_full_returns_none() {
        let mut set = GenBitSet::new(3);
        for _ in 0..3 {
            assert!(set.acquire().is_some());
        }
        assert!(set.acquire().is_none());
    }

    #[test]
    fn zero_sized_set_cannot_acquire() {
        let mut set = GenBitSet::new(0);
        assert_eq!(set.capacity(), 0);
        assert!(set.acquire().is_none());
    }

    #[test]
    fn acquired_bits_are_unique() {
        let mut set = set();
        let mut indices: Vec<u32> = (0..SIZE)
            .map(|_| {
                let b = set.acquire().unwrap();
                set.downgrade(b).unwrap()
            })
            .collect();
        indices.sort_unstable();
        indices.dedup();
        assert_eq!(indices.len(), SIZE);
    }

    #[test]
    fn double_release_fails() {
        let mut set = set();
        let bit = set.acquire().unwrap();
        set.release(bit).unwrap();

        assert!(matches!(set.release(bit), Err(Error::BitReleased { .. })));
        // The failed release must not bump the generation a second time.
        let index = set.bits.zeroes().next().unwrap();
        assert!(set.generations[index] <= 1);
    }

    #[test]
    fn stale_handle_is_invalid_after_slot_reuse() {
        let mut set = GenBitSet::new(1);

        let old = set.acquire().unwrap();
        set.release(old).unwrap();
        let new = set.acquire().unwrap();

        // Same slot, different generation.
        assert_ne!(old, new);
        assert!(set.contains(new));
        assert!(
            !set.contains(old),
            "ABA problem: stale handle must not be valid"
        );
        assert!(matches!(set.release(old), Err(Error::BitReleased { .. })));
        assert!(
            set.contains(new),
            "stale release must not affect the new owner"
        );
    }

    #[test]
    fn generation_increments_every_cycle() {
        let mut set = GenBitSet::new(1);
        for expected in 0..5u32 {
            let bit = set.acquire().unwrap();
            assert_eq!(set.generations[0], expected);
            set.release(bit).unwrap();
        }
        assert_eq!(set.generations[0], 5);
    }

    #[test]
    fn release_makes_slot_available_again() {
        let mut set = GenBitSet::new(1);
        let bit = set.acquire().unwrap();
        assert!(set.acquire().is_none());
        set.release(bit).unwrap();
        assert!(set.acquire().is_some());
    }

    #[test]
    fn upgrade_after_reacquire_yields_new_generation() {
        let mut set = GenBitSet::new(1);
        let old = set.acquire().unwrap();
        let index = set.downgrade(old).unwrap();
        set.release(old).unwrap();
        let new = set.acquire().unwrap();

        let upgraded = set.upgrade(index as usize).unwrap();
        assert_eq!(upgraded, new);
        assert_ne!(upgraded, old);
    }
}
