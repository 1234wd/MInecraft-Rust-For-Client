//! Port of: (no Java counterpart -- see module docs)
//! Java class(es): `java.util.BitSet` (partial: the subset `CarvingMask` uses)
//! Status: VERIFIED
//!
//! Session 22. `new BitSet(nbits)`, `BitSet.valueOf(long[])`, `set(int)`, `get(int)`, `stream()` and
//! `toLongArray()`, with the JDK's exception texts.
//!
//! # What matters
//!
//! * A negative index throws `IndexOutOfBoundsException("bitIndex < 0: " + i)` from both `set` and
//!   `get`; a negative initial size throws `NegativeArraySizeException("nbits < 0: " + n)`. The port
//!   panics with those texts (`DESIGN_DECISIONS.md#runtime-exceptions`).
//! * `get` beyond the last word is simply `false`; `set` grows the word array.
//! * `toLongArray()` returns the words up to the LAST NON-ZERO one (`wordsInUse`), and
//!   `valueOf(long[])` drops trailing zero words on the way in -- so `valueOf([5, 0, 0])` round-trips
//!   to `[5]`. The initial `nbits` only sizes the allocation and is never observable through these
//!   methods.
//! * `stream()` yields the set bits in ascending order, as `int`s.
//!
//! Clearing, `size()`, `length()` and the rest of the class are not ported.
//!
//! Checked against JDK 25 by the `CARVING` group of `_porting/test-data/small_probe.txt`
//! (`_porting/tests/session22.rs`), through `CarvingMask`.

/// Port of the used subset of `java.util.BitSet`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BitSet {
    /// Always trimmed: the last word, if any, is non-zero (the JDK's `wordsInUse` invariant).
    words: Vec<u64>,
}

fn check_index(bit_index: i32) {
    if bit_index < 0 {
        panic!("java.lang.IndexOutOfBoundsException: bitIndex < 0: {bit_index}");
    }
}

impl BitSet {
    /// Port of `new BitSet(int nbits)`. The size is a capacity hint only, but a negative one throws.
    pub fn with_nbits(nbits: i32) -> Self {
        if nbits < 0 {
            panic!("java.lang.NegativeArraySizeException: nbits < 0: {nbits}");
        }
        BitSet { words: Vec::with_capacity((nbits as usize).div_ceil(64)) }
    }

    /// Port of `BitSet.valueOf(long[] longs)`: a copy, with trailing zero words dropped.
    pub fn value_of(longs: &[i64]) -> Self {
        let mut words: Vec<u64> = longs.iter().map(|&w| w as u64).collect();
        while words.last() == Some(&0) {
            words.pop();
        }
        BitSet { words }
    }

    /// Port of `set(int bitIndex)`.
    pub fn set(&mut self, bit_index: i32) {
        check_index(bit_index);
        let word = (bit_index >> 6) as usize;
        if word >= self.words.len() {
            self.words.resize(word + 1, 0);
        }
        self.words[word] |= 1u64 << (bit_index & 63);
    }

    /// Port of `get(int bitIndex)`.
    pub fn get(&self, bit_index: i32) -> bool {
        check_index(bit_index);
        let word = (bit_index >> 6) as usize;
        word < self.words.len() && (self.words[word] >> (bit_index & 63)) & 1 != 0
    }

    /// Port of `stream()`: the set bits, ascending.
    pub fn stream(&self) -> impl Iterator<Item = i32> + '_ {
        self.words.iter().enumerate().flat_map(|(w, &bits)| {
            (0..64).filter(move |b| (bits >> b) & 1 != 0).map(move |b| (w as i32) * 64 + b)
        })
    }

    /// Port of `toLongArray()`: the words up to the last non-zero one.
    pub fn to_long_array(&self) -> Vec<i64> {
        self.words.iter().map(|&w| w as i64).collect()
    }
}
