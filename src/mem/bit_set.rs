type ChunkType = u64;

pub struct BitSet {
    inner: Vec<ChunkType>,
    /// Length in bits.
    bit_len: usize,
}

// TODO:
// pub enum Op {
//     And,
//     Xor,
//     Or,
// }

pub struct BitIterator<'a> {
    /// All the `BitVec`s that need to be comined when iterating.
    bits: Vec<&'a BitSet>,
    /// Result of combining the chunk `i` is in right now in `bits`.
    /// Updated each time `i` crosses a chunk boundry.
    /// Exists to not recalculate the result every time.
    chunk_result: ChunkType,
    /// The smallest length among `bits`
    min_len: usize,
    /// The index in all the `bits`
    i: usize,
}

impl<'a> BitIterator<'a> {
    pub fn new(bits: &'a BitSet) -> Self {
        let min_len = bits.len();
        Self {
            bits: vec![bits],
            min_len,
            chunk_result: 0, // Will be updated upon iteration.
            i: 0,
        }
    }

    pub fn and(mut self, bits: &'a BitSet) -> Self {
        self.min_len = self.min_len.min(bits.len());
        self.bits.push(bits);
        self
    }
}

impl Iterator for BitIterator<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.min_len {
            return None;
        }

        let bit_index = self.i % (ChunkType::BITS as usize);

        // Recaulcuate `chunk_result`
        if bit_index == 0 {
            let chunk_index = self.i / (ChunkType::BITS as usize);

            self.chunk_result = !0;
            for bits in &self.bits {
                self.chunk_result &= bits.inner[chunk_index];
            }
        }

        self.i += 1;

        Some(0 != ((self.chunk_result >> bit_index) & 1))
    }
}

impl BitSet {
    pub fn new(size: usize, intial_values: bool) -> Self {
        let mut s = Self {
            inner: Vec::new(),
            bit_len: 0,
        };
        s.resize(size, intial_values);
        s
    }

    pub fn iter_bits(&self) -> BitIterator<'_> {
        BitIterator::new(self)
    }

    pub fn get_unchecked(&self, i: usize) -> bool {
        let bit_index = i % (ChunkType::BITS as usize);
        let chunk_index = i / (ChunkType::BITS as usize);

        0 != ((self.inner[chunk_index] >> bit_index) & 1)
    }

    pub fn set_unchecked(&mut self, i: usize, value: bool) {
        let bit_index = i % (ChunkType::BITS as usize);
        let chunk_index = i / (ChunkType::BITS as usize);

        if value {
            self.inner[chunk_index] |= 1 << bit_index;
        } else {
            self.inner[chunk_index] &= !(1 << bit_index);
        }
    }

    pub fn get(&self, i: usize) -> Option<bool> {
        if i >= self.bit_len {
            None
        } else {
            Some(self.get_unchecked(i))
        }
    }

    pub fn first_zero(&self) -> Option<usize> {
        for (i, &chunk) in self.inner.iter().enumerate() {
            if let Some(lowest_zero) = (!chunk).lowest_one() {
                return Some((lowest_zero as usize) + i * (ChunkType::BITS as usize));
            }
        }
        None
    }

    pub fn set(&mut self, i: usize, value: bool) {
        if i >= self.bit_len {
            panic!("Out of bounds access {} len {}", i, self.bit_len);
        } else {
            self.set_unchecked(i, value);
        }
    }

    pub fn resize(&mut self, new_size: usize, intial_values: bool) {
        let actual_initial_values = if intial_values { !0 } else { 0 };
        let actual_new_size = new_size.next_multiple_of(ChunkType::BITS as usize) / ChunkType::BITS as usize;
        self.inner.resize(actual_new_size, actual_initial_values);
        self.bit_len = new_size;
    }

//     pub fn push(&mut self, b: bool) {
//         let bit_index = self.bit_len % (ChunkType::BITS as usize);
//         let chunk_index = self.bit_len / (ChunkType::BITS as usize);
//
//         // Recaulcuate `chunk_result`
//         if bit_index == 0 {
//             self.inner.push(0);
//         }
//
//         if b {
//             self.inner[chunk_index] |= 1 << bit_index;
//         } else {
//             self.inner[chunk_index] &= !(1 << bit_index);
//         }
//
//         self.bit_len += 1;
//     }

    pub fn len(&self) -> usize {
        self.bit_len
    }
}

impl<const N: usize> From<[bool; N]> for BitSet {
    fn from(bools: [bool; N]) -> Self {
        Self::from(bools.as_slice())
    }
}

// TODO: This is too quick and dirty. Optimize later.
impl From<&[bool]> for BitSet {
    fn from(bools: &[bool]) -> Self {
        let mut bits = BitSet::new(bools.len(), false);
        for (i, &b) in bools.iter().enumerate() {
            bits.set(i, b);
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    const EXAMPLE_BOOLS_0: [bool; 13] = [
        true, false, true, true, true, false, false, false, false, true, true, false, true,
    ];
    const EXAMPLE_BOOLS_1: [bool; 13] = [
        false, false, true, true, false, false, true, true, true, false, false, true, false,
    ];

    /// Ensures the `From<&[bool]>` works and that `get` and `get_unchecked` work.
    #[test]
    fn from_bools_and_assert_gets() {
        let bools = EXAMPLE_BOOLS_0.repeat(100);
        let bit_vec: BitSet = bools.clone().as_slice().into();

        for (i, b) in bools.into_iter().enumerate() {
            assert_eq!(bit_vec.get_unchecked(i), b);
            assert_eq!(bit_vec.get(i).unwrap(), b);
        }
    }

    #[test]
    fn out_of_bounds_get() {
        let bools = EXAMPLE_BOOLS_0.repeat(100);
        let bit_vec: BitSet = bools.clone().as_slice().into();

        let result = panic::catch_unwind(|| {
            bit_vec.get_unchecked(123123213);
        });

        assert!(result.is_err(), "Was supposed to be out of bounds access");

        assert_eq!(bit_vec.get(32312312), None);
    }

    /// Ensures that a basic iter_bits() works over a BitVec.
    #[test]
    fn iterate_over_1_vecs() {
        let bools = EXAMPLE_BOOLS_0.repeat(32);
        let bit_vec: BitSet = bools.clone().as_slice().into();

        for (bit_vec_b, bools_b) in bit_vec.iter_bits().zip(bools.into_iter()) {
            assert_eq!(bit_vec_b, bools_b);
        }
    }

    /// Creates two BitVecs with 2 different examples.
    ///
    /// Ensures that an iterator that iterates over both with an and operation
    /// correctly produces the intersection result.
    #[test]
    fn iterate_over_2_vecs_with_and() {
        let bools_0 = EXAMPLE_BOOLS_0.repeat(32);
        let bit_vec_0: BitSet = bools_0.clone().as_slice().into();

        let bools_1 = EXAMPLE_BOOLS_1.repeat(32);
        let bit_vec_1: BitSet = bools_1.clone().as_slice().into();

        let iter = bit_vec_0.iter_bits().and(&bit_vec_1);

        // Don't let the ugly zip scare you, it just makes all iterators run in parallel.
        for ((iter_b, bools_0_b), bools_1_b) in
            iter.zip(bools_0.into_iter()).zip(bools_1.into_iter())
        {
            assert_eq!(iter_b, bools_0_b & bools_1_b);
        }
    }
}
