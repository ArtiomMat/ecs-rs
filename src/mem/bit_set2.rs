use smallvec::{SmallVec, smallvec};

use crate::mem::bit_set2::Op::Intersection;

pub type Block = u64;

const fn block_bits() -> usize {
    Block::BITS as usize
}

fn bits_to_blocks(bits_n: usize) -> usize {
    bits_n.div_ceil(Block::BITS as usize)
}

pub struct BitSet2 {
    blocks: Vec<Block>,
    capacity: usize,
}

impl BitSet2 {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            capacity: 0,
        }
    }

    /// The format is guaranteed to be:
    /// - Block `i` contains bits `(i * Block::BITS)..(i * Block::BITS + Block::BITS)`.
    /// - Each block's LSB is the lowest index bit, while HSB is the highest index.
    ///
    /// Shifting are done with Rust's plain shift operators.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut bitset = Self::new();
        bitset.grow(capacity);
        bitset
    }

    pub fn grow(&mut self, new_capacity: usize) {
        let blocks_size = bits_to_blocks(new_capacity);
        self.blocks.resize(blocks_size, 0);
        self.capacity = new_capacity
    }

    /// # Panics
    /// If the `bit` is out of bounds.
    pub fn set(&mut self, bit: usize, value: bool) {
        assert!(
            self.capacity > bit,
            "Bit {} out of bounds {}",
            bit,
            self.capacity
        );

        let mask = 1 << (bit % block_bits());
        if value {
            self.blocks[bit / block_bits()] |= mask;
        } else {
            self.blocks[bit / block_bits()] &= !mask;
        }
    }

    /// Sets `bit` to 1.
    ///
    /// # Panics
    /// If the `bit` is out of bounds.
    pub fn insert(&mut self, bit: usize) {
        self.set(bit, true)
    }

    /// Sets `bit` to 0.
    ///
    /// # Panics
    /// If the `bit` is out of bounds.
    pub fn remove(&mut self, bit: usize) {
        self.set(bit, false)
    }

    /// Returns the bit value as a boolean.
    /// Always returns `false` for out of bounds.
    pub fn contains(&self, bit: usize) -> bool {
        if bit >= self.capacity {
            return false;
        }
        let block_value = 1 << (bit % block_bits());
        self.blocks[bit / block_bits()] & block_value != 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn zeroes<'a>(&'a self) -> MultiBits<'a> {
        // TODO: Change to Zeroes when you implement
        MultiBits::zeroes(self)
    }

    pub fn ones<'a>(&'a self) -> MultiBits<'a> {
        // TODO: Change to Ones when you implement
        MultiBits::ones(self)
    }
}

pub struct Bits<'a> {
    set: &'a BitSet2,
    block: usize,
    bit: usize,
}

#[derive(Clone, Copy)]
pub enum Op {
    /// OR
    Union,
    /// Negated AND
    Difference,
    /// AND
    Intersection,
}

#[derive(Clone, Copy)]
pub struct MultiBitsPart<'a> {
    set: &'a BitSet2,
    op: Op,
}

pub struct MultiBits<'a> {
    parts: SmallVec<[MultiBitsPart<'a>; 8]>,
    /// Bit index shared for all bitsets
    bit_index: usize,
    /// The value of the current block cached after operations
    current_block: Block,
    /// The minimum capacity from all the sets
    min_capacity: usize,
}

impl<'a> MultiBits<'a> {
    /// `op` happens on a virtual all-ones set.
    pub fn new(set: &'a BitSet2, op: Op) -> Self {
        Self {
            parts: smallvec![MultiBitsPart {
                set,
                op
            }],
            bit_index: 0,
            current_block: 0,
            min_capacity: set.capacity,
        }
    }

    /// As if you are iterating over the ones in the set
    pub fn ones(set: &'a BitSet2) -> Self {
        Self::new(set, Op::Intersection)
    }

    /// As if you are iterating over the zeroes in the set
    pub fn zeroes(set: &'a BitSet2) -> Self {
        Self::new(set, Op::Difference)
    }

    pub fn push(&mut self, set: &'a BitSet2, op: Op) {
        if set.capacity < self.min_capacity {
            self.min_capacity = set.capacity;
        }
        self.parts.push(MultiBitsPart { set, op });
    }

    fn recalculate_current_block(&mut self) {
        let block_index = self.bit_index / Block::BITS as usize;

        self.current_block = !0;

        for &MultiBitsPart { set, op } in self.parts.iter() {
            let part_block = set.blocks[block_index];
            match op {
                Op::Union => self.current_block |= part_block,
                Op::Difference => self.current_block &= !part_block,
                Op::Intersection => self.current_block &= part_block,
            }
        }
    }

    /// Returns `None` if reached the end of the minimum capacity and iterator should return `None`.
    #[must_use]
    fn skip_while_zero(&mut self) -> Option<()> {
        // As long as self.block_after_ops turned up 0 we can skip it and keep going
        while self.current_block == 0 {
            self.bit_index += Block::BITS as usize;
            if self.bit_index >= self.min_capacity {
                return None;
            }

            self.recalculate_current_block();
        }

        Some(())
    }
}

impl Iterator for MultiBits<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bit_index = self.bit_index;
            if bit_index >= self.min_capacity {
                return None;
            }

            // Recalculate self.block_after_ops if we entered a new block
            if self.bit_index % Block::BITS as usize == 0 {
                self.recalculate_current_block();
                self.skip_while_zero()?;
            }

            self.bit_index += 1;

            if (self.current_block >> (bit_index % Block::BITS as usize)) & 1 == 1 {
                return Some(bit_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Bit 105 out of bounds 102")]
    fn insert_out_of_bounds() {
        let mut bitset = BitSet2::with_capacity(102);
        bitset.insert(0);
        bitset.insert(66);
        bitset.insert(55);
        bitset.insert(Block::BITS as usize - 1);
        bitset.insert(Block::BITS as usize);
        bitset.insert(Block::BITS as usize + 1);
        bitset.insert(6);
        bitset.insert(101);
        bitset.insert(105);
    }

    #[test]
    fn remove_removed() {
        let mut bitset = BitSet2::with_capacity(102);
        bitset.insert(0);
        bitset.insert(66);
        bitset.insert(55);
        bitset.remove(55);
        bitset.remove(55);

        assert!(!bitset.contains(55));
    }

    #[test]
    fn insert_and_contains() {
        const INSERTS: [usize; 8] = [
            0,
            66,
            55,
            Block::BITS as usize - 1,
            Block::BITS as usize,
            Block::BITS as usize + 1,
            6,
            101,
        ];

        let mut bitset = BitSet2::with_capacity(102);

        for bit in INSERTS {
            bitset.insert(bit);
        }

        for bit in INSERTS {
            assert!(bitset.contains(bit));
        }
    }

    #[test]
    fn insert_and_remove_and_contains() {
        const INSERTS: [usize; 8] = [
            0,
            66,
            55,
            Block::BITS as usize - 1,
            Block::BITS as usize,
            Block::BITS as usize + 1,
            6,
            101,
        ];
        const REMOVES: [usize; 4] = [0, 101, 55, 56];
        const CONTAINS: [usize; 5] = [
            66,
            Block::BITS as usize - 1,
            Block::BITS as usize,
            Block::BITS as usize + 1,
            6,
        ];

        let mut bitset = BitSet2::with_capacity(102);

        for bit in INSERTS {
            bitset.insert(bit);
        }

        for bit in REMOVES {
            bitset.remove(bit);
        }

        for bit in CONTAINS {
            assert!(bitset.contains(bit));
        }
    }

    #[test]
    fn multi_bits_0() {
        let mut base = BitSet2::with_capacity(100);
        base.insert(0);
        base.insert(1);
        base.insert(21);
        base.insert(22);
        base.insert(23);
        base.insert(55);
        base.insert(99);
        let mut neg_and = BitSet2::with_capacity(100);
        neg_and.insert(22);
        neg_and.insert(23);
        let mut and = BitSet2::with_capacity(100);
        and.insert(21);
        and.insert(64);
        and.insert(99);
        let mut or = BitSet2::with_capacity(100);
        or.insert(56);
        or.insert(23);

        // Will get 21, 23, 56, 99

        let mut iter = MultiBits::ones(&base);
        iter.push(&neg_and, Op::Difference);
        iter.push(&and, Op::Intersection);
        iter.push(&or, Op::Union);

        let mut collected = iter.collect::<Vec<usize>>();
        collected.sort();
        assert_eq!(collected, [21, 23, 56, 99]);
    }

    #[test]
    fn multi_bits_1() {
        // Spans three blocks, mixing NegIntersection, Intersection and Union
        let mut base = BitSet2::with_capacity(130);
        base.insert(3);
        base.insert(64);
        base.insert(65);
        base.insert(100);
        base.insert(127);
        base.insert(129);

        let mut neg_and = BitSet2::with_capacity(130);
        neg_and.insert(64);
        neg_and.insert(129);

        let mut and = BitSet2::with_capacity(130);
        and.insert(3);
        and.insert(5);
        and.insert(65);
        and.insert(100);
        and.insert(129);

        let mut or = BitSet2::with_capacity(130);
        or.insert(128);

        // base & !neg_and = 3, 65, 100, 127
        // ... & and       = 3, 65, 100
        // ... | or        = 3, 65, 100, 128

        let mut iter = MultiBits::ones(&base);
        iter.push(&neg_and, Op::Difference);
        iter.push(&and, Op::Intersection);
        iter.push(&or, Op::Union);

        let collected = iter.collect::<Vec<usize>>();
        assert_eq!(collected, [3, 65, 100, 128]);
    }

    #[test]
    fn multi_bits_skips_empty_blocks() {
        // Block 1 becomes empty after the ops, and block 3 is empty from the start,
        // so both should be skipped
        let mut base = BitSet2::with_capacity(200);
        base.insert(10);
        base.insert(70);
        base.insert(150);
        base.insert(190);

        let mut and = BitSet2::with_capacity(200);
        and.insert(10);
        and.insert(75);
        and.insert(150);
        and.insert(190);

        let mut neg_and = BitSet2::with_capacity(200);
        neg_and.insert(150);

        // base & and      = 10, 150, 190   (70 and 75 don't overlap, block 1 is empty)
        // ... & !neg_and  = 10, 190

        let mut iter = MultiBits::ones(&base);
        iter.push(&and, Op::Intersection);
        iter.push(&neg_and, Op::Difference);

        let collected = iter.collect::<Vec<usize>>();
        assert_eq!(collected, [10, 190]);
    }

    #[test]
    fn multi_bits_last_bit_of_full_block() {
        let mut base = BitSet2::with_capacity(64);
        base.insert(0);
        base.insert(63);

        let collected = MultiBits::ones(&base).collect::<Vec<usize>>();
        assert_eq!(collected, [0, 63]);
    }

    #[test]
    fn multi_bits_empty_set() {
        let base = BitSet2::with_capacity(0);

        let collected = MultiBits::ones(&base).collect::<Vec<usize>>();
        assert!(collected.is_empty());
    }

    #[test]
    fn multi_bits_smaller_pushed_set() {
        let mut base = BitSet2::with_capacity(130);
        base.insert(3);
        base.insert(100);

        let mut small = BitSet2::with_capacity(64);
        small.insert(3);

        let mut iter = MultiBits::ones(&base);
        iter.push(&small, Op::Intersection);

        let collected = iter.collect::<Vec<usize>>();
        assert_eq!(collected, [3]);
    }

    #[test]
    fn multi_bits_does_not_leak_past_capacity() {
        let mut base = BitSet2::with_capacity(70);
        base.insert(69);

        let mut wide = BitSet2::with_capacity(128);
        wide.insert(70);

        let mut iter = MultiBits::ones(&base);
        iter.push(&wide, Op::Union);

        let collected = iter.collect::<Vec<usize>>();
        assert_eq!(collected, [69]);
    }
}
