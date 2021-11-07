use std::cmp::Ordering;
use std::fmt::{Binary, Debug, Display, Error, Formatter, Write};
use std::iter::{Chain, FromIterator};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index};
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::ptr;

use crate::{init_lib, BVector, _check_res};

impl BVector {
    /// Create a new bit-vector container with runtime compression of bits
    pub fn new() -> BVector {
        init_lib();
        let mut handle = ptr::null_mut();
        let res;
        // TODO: BM_bvector_construct can fail from memory allocations,
        // or from passing a null ptr as handle.
        // If we follow stdlib semantics, a memory allocation error
        // show be a panic, and would also avoid changing the method signature
        // to return a Result.
        unsafe {
            res = bitmagic_sys::BM_bvector_construct(&mut handle, 0);
            // TODO: potentially call BM_bvector_init too,
            // so we can call the _no_check() functions?
        };
        _check_res(res);
        BVector { handle }
    }

    /// Create a new bit-vector container with capacity (maximum number of
    /// allowed bits).
    ///
    /// BitMagic recommends ALWAYS setting the size to maximum, so it is
    /// preferable to use `new` instead.
    ///
    /// [`new`]: BVector::new
    pub fn with_capacity(capacity: usize) -> BVector {
        init_lib();
        let mut handle = ptr::null_mut();
        let res;
        // TODO: BM_bvector_construct can fail from memory allocations,
        // or from passing a null ptr as handle.
        // If we follow stdlib semantics, a memory allocation error
        // show be a panic, and would also avoid changing the method signature
        // to return a Result.
        unsafe {
            res = bitmagic_sys::BM_bvector_construct(&mut handle, capacity as u32);
            // TODO: potentially call BM_bvector_init too,
            // so we can call the _no_check() functions?
        };
        _check_res(res);
        BVector { handle }
    }

    /// Grow capacity to bits, all new bits initialized to zero
    pub fn grow(&mut self, bits: usize) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_set_size(self.handle, bits as u32);
            // TODO: check res
        };
        _check_res(res);
    }

    /// Return the length of the BVector in bits.
    pub fn len(&self) -> usize {
        let mut size = 0;
        let res;
        // TODO: size returns the full size (4 GB?), but fixedbitset
        // has a fixed size and is based on the capacity.
        unsafe {
            res = bitmagic_sys::BM_bvector_get_size(self.handle, &mut size);
            // TODO: check res
        };
        _check_res(res);
        // TODO: size set to maximum, return 0 for now
        if size == 4294967295 {
            0
        } else {
            size as usize
        }
    }

    /// Check if any bits are set
    pub fn is_empty(&self) -> bool {
        // TODO: check len, or if any bit is set?
        self.len() == 0
    }

    /// Return **true** if the bit is enabled in the **BVector**,
    /// **false** otherwise.
    ///
    /// Note: bits outside the capacity are always disabled.
    ///
    /// Note: Also available with index syntax: `bvector[bit]`.
    pub fn contains(&self, bit: usize) -> bool {
        if bit >= self.len() {
            return false;
        }

        let mut pval = 0;
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_get_bit(self.handle, bit as u32, &mut pval);
        };
        _check_res(res);
        pval == 1
    }

    /// Clear all bits.
    pub fn clear(&mut self) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_clear(self.handle, 0);
        };
        // TODO: check res, check if 0 is actually NOT free mem
        _check_res(res);
    }

    /// Enable `bit`.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn insert(&mut self, bit: usize) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_set_bit_no_check(self.handle, bit as u32);
        };
        _check_res(res);
    }

    /// Enable `bit`, and return its previous value.
    ///
    /// **Panics** if **bit** is out of bounds.
    pub fn put(&mut self, bit: usize) -> bool {
        let mut pval = 0;

        let mut res;
        unsafe {
            res = bitmagic_sys::BM_bvector_get_bit(self.handle, bit as u32, &mut pval);
            _check_res(res);
            res = bitmagic_sys::BM_bvector_set_bit_no_check(self.handle, bit as u32);
            _check_res(res);
        };
        pval == 1
    }

    /// Toggle `bit` (inverting its state).
    ///
    /// ***Panics*** if **bit** is out of bounds
    pub fn toggle(&mut self, bit: usize) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_flip_bit(self.handle, bit as u32);
        };
        _check_res(res);
    }

    /// **Panics** if **bit** is out of bounds.
    pub fn set(&mut self, bit: usize, enabled: bool) {
        let val = if enabled { 1 } else { 0 };

        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_set_bit(self.handle, bit as u32, val);
        };
        _check_res(res);
    }

    /// Copies boolean value from specified bit to the specified bit.
    ///
    /// **Panics** if **to** is out of bounds.
    pub fn copy_bit(&mut self, from: usize, to: usize) {
        self.set(to, self.contains(from));
    }

    #[inline]
    fn parse_range<T: IndexRange>(&self, range: T) -> (u32, u32) {
        let len = self.len();

        let start = range.start().unwrap_or(0) as u32;

        let end = match range.end() {
            Some(x) => {
                if x <= len {
                    if x == 0 {
                        0
                    } else if x as u32 == start {
                        return (start, 0);
                    } else {
                        x - 1
                    }
                } else {
                    panic!("Range extends past the end of the vector")
                }
            }
            None => len - 1,
        } as u32;

        assert!(end >= start);

        (start, end)
    }

    /// Count the number of set bits in the given bit range.
    ///
    /// Use `..` to count the whole content of the bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn count_ones<T: IndexRange>(&self, range: T) -> usize {
        let res;
        let mut pcount = 0;
        let (start, end) = self.parse_range(range);

        unsafe {
            res = bitmagic_sys::BM_bvector_count_range(self.handle, start, end, &mut pcount);
        }
        _check_res(res);
        pcount as usize
    }

    /// Sets every bit in the given range to the given state (`enabled`)
    ///
    /// Use `..` to set the whole bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    pub fn set_range<T: IndexRange>(&mut self, range: T, enabled: bool) {
        let (start, end) = self.parse_range(range);
        let val = if enabled { 1 } else { 0 };

        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_set_range(self.handle, start, end, val);
        }
        _check_res(res);
    }

    /// Enables every bit in the given range.
    ///
    /// Use `..` to make the whole bitset ones.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    pub fn insert_range<T: IndexRange>(&mut self, range: T) {
        self.set_range(range, true);
    }

    /// Toggles (inverts) every bit in the given range.
    ///
    /// Use `..` to toggle the whole bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    pub fn toggle_range<T: IndexRange>(&mut self, range: T) {
        let (start, end) = self.parse_range(range);
        for i in start..=end {
            self.toggle(i as usize);
        }
    }

    /// Iterates over all enabled bits.
    ///
    /// Iterator element is the index of the `1` bit, type `usize`.
    pub fn ones(&self) -> Ones {
        Ones {
            current_bit_idx: None,
            bv: self,
        }
    }

    /// Returns a lazy iterator over the intersection of two `BVector`s
    pub fn intersection<'a>(&'a self, other: &'a BVector) -> Intersection<'a> {
        Intersection {
            iter: self.ones(),
            other,
        }
    }

    /// Returns a lazy iterator over the union of two `BVector`s.
    pub fn union<'a>(&'a self, other: &'a BVector) -> Union<'a> {
        Union {
            iter: self.ones().chain(other.difference(self)),
        }
    }

    /// Returns a lazy iterator over the difference of two `BVector`s. The difference of `a`
    /// and `b` is the elements of `a` which are not in `b`.
    pub fn difference<'a>(&'a self, other: &'a BVector) -> Difference<'a> {
        Difference {
            iter: self.ones(),
            other,
        }
    }

    /// Returns a lazy iterator over the symmetric difference of two `BVector`s.
    /// The symmetric difference of `a` and `b` is the elements of one, but not both, sets.
    pub fn symmetric_difference<'a>(&'a self, other: &'a BVector) -> SymmetricDifference<'a> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }

    /// In-place union of two `BVector`s.
    ///
    /// On calling this method, `self`'s capacity may be increased to match `other`'s.
    pub fn union_with(&mut self, other: &BVector) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_combine_OR(self.handle, other.handle);
        }
        _check_res(res);
    }

    /// In-place intersection of two `BVector`s.
    ///
    /// On calling this method, `self`'s capacity will remain the same as before.
    pub fn intersect_with(&mut self, other: &BVector) {
        //let len = self.len();

        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_combine_AND(self.handle, other.handle);
            _check_res(res);

            // TODO: fixedbitset truncates to size of self, while bitmagic doesn't
            //res = bitmagic_sys::BM_bvector_set_size(self.handle, len as u32);
            //_check_res(res);
        }
    }

    /// In-place difference of two `BVector`s.
    ///
    /// On calling this method, `self`'s capacity will remain the same as before.
    pub fn difference_with(&mut self, other: &BVector) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_combine_SUB(self.handle, other.handle);
        }
        _check_res(res);
    }

    /// In-place symmetric difference of two `BVector`s.
    ///
    /// On calling this method, `self`'s capacity may be increased to match `other`'s.
    pub fn symmetric_difference_with(&mut self, other: &BVector) {
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_combine_XOR(self.handle, other.handle);
            // TODO: check res
        }
        _check_res(res);
    }

    /// Returns `true` if `self` has no elements in common with `other`. This
    /// is equivalent to checking for an empty intersection.
    pub fn is_disjoint(&self, other: &BVector) -> bool {
        self.intersection_count(other) == 0
    }

    /// Returns `true` if the set is a subset of another, i.e. `other` contains
    /// at least all the values in `self`.
    pub fn is_subset(&self, other: &BVector) -> bool {
        // TODO: check if bitmagic has something equivalent
        self.ones().all(|pos| other.contains(pos))
    }

    /// Returns `true` if the set is a superset of another, i.e. `self` contains
    /// at least all the values in `other`.
    pub fn is_superset(&self, other: &BVector) -> bool {
        // TODO: check if bitmagic has something equivalent
        other.ones().all(|pos| self.contains(pos))
    }
}

impl Binary for BVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if f.alternate() {
            f.write_str("0b")?;
        }

        for i in 0..self.len() {
            if self[i] {
                f.write_char('1')?;
            } else {
                f.write_char('0')?;
            }
        }

        Ok(())
    }
}

impl Display for BVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Binary::fmt(&self, f)
    }
}

impl Debug for BVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Binary::fmt(&self, f)
    }
}

impl Drop for BVector {
    fn drop(&mut self) {
        unsafe {
            bitmagic_sys::BM_bvector_free(self.handle);
        }
    }
}

impl PartialOrd for BVector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut pres = 0;

        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_compare(self.handle, other.handle, &mut pres);
        }
        _check_res(res);

        match pres {
            0 => Some(Ordering::Equal),
            -1 => Some(Ordering::Less),
            1 => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl PartialEq for BVector {
    fn eq(&self, other: &Self) -> bool {
        let mut pres = 0;

        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_compare(self.handle, other.handle, &mut pres);
            // TODO: check res
        }
        _check_res(res);

        pres == 0
    }
}

impl Default for BVector {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator producing elements in the difference of two sets.
///
/// This struct is created by the [`BVector::difference`] method.
pub struct Difference<'a> {
    iter: Ones<'a>,
    other: &'a BVector,
}

impl<'a> Iterator for Difference<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for nxt in &mut self.iter {
            if !self.other.contains(nxt) {
                return Some(nxt);
            }
        }
        None
    }
}

/// An iterator producing elements in the symmetric difference of two sets.
///
/// This struct is created by the [`BVector::symmetric_difference`] method.
pub struct SymmetricDifference<'a> {
    iter: Chain<Difference<'a>, Difference<'a>>,
}

impl<'a> Iterator for SymmetricDifference<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator producing elements in the intersection of two sets.
///
/// This struct is created by the [`BVector::intersection`] method.
pub struct Intersection<'a> {
    iter: Ones<'a>,
    other: &'a BVector,
}

impl<'a> Iterator for Intersection<'a> {
    type Item = usize; // the bit position of the '1'

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for nxt in &mut self.iter {
            if self.other.contains(nxt) {
                return Some(nxt);
            }
        }
        None
    }
}

/// An iterator producing elements in the union of two sets.
///
/// This struct is created by the [`BVector::union`] method.
pub struct Union<'a> {
    iter: Chain<Ones<'a>, Difference<'a>>,
}

impl<'a> Iterator for Union<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator producing the indices of the set bit in a set.
///
/// This struct is created by the [`BVector::ones`] method.
pub struct Ones<'a> {
    current_bit_idx: Option<u32>,
    bv: &'a BVector,
}

impl<'a> Iterator for Ones<'a> {
    type Item = usize; // the bit position of the '1'

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut pnext = 0;

        let mut res;
        if self.current_bit_idx.is_none() {
            self.current_bit_idx = Some(0);

            let mut pnext = 0;
            unsafe {
                res = bitmagic_sys::BM_bvector_get_bit(self.bv.handle, 0, &mut pnext);
            }
            _check_res(res);
            if pnext == 1 {
                return Some(0);
            }
        }

        unsafe {
            res = bitmagic_sys::BM_bvector_get_next(
                self.bv.handle,
                self.current_bit_idx.unwrap(),
                &mut pnext,
            );
        }
        _check_res(res);

        if pnext > self.current_bit_idx.unwrap() {
            self.current_bit_idx = Some(pnext);
            Some(pnext as usize)
        } else {
            None
        }
    }
}

impl Clone for BVector {
    #[inline]
    fn clone(&self) -> Self {
        let mut handle = ptr::null_mut();

        // TODO: BM_bvector_construct_copy can fail from memory allocations,
        // or from passing a null ptr as handle.
        // If we follow stdlib semantics, a memory allocation error
        // show be a panic, and would also avoid changing the method signature
        // to return a Result.
        let res;
        unsafe {
            res = bitmagic_sys::BM_bvector_construct_copy(&mut handle, self.handle);
            _check_res(res);
            // TODO: potentially call BM_bvector_init too,
            // so we can call the _no_check() functions?
        };
        BVector { handle }
    }
}

/// Return **true** if the bit is enabled in the bitset,
/// or **false** otherwise.
///
/// Note: bits outside the capacity are always disabled, and thus
/// indexing a BVector will not panic.
impl Index<usize> for BVector {
    type Output = bool;

    #[inline]
    fn index(&self, bit: usize) -> &bool {
        if self.contains(bit) {
            &true
        } else {
            &false
        }
    }
}

/// Sets the bit at index **i** to **true** for each item **i** in the input **src**.
impl Extend<usize> for BVector {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, src: I) {
        let iter = src.into_iter();
        for i in iter {
            if i >= self.len() {
                self.grow(i + 1);
            }
            self.put(i);
        }
    }
}

/// Return a BVector containing bits set to **true** for every bit index in
/// the iterator, other bits are set to **false**.
impl FromIterator<usize> for BVector {
    fn from_iter<I: IntoIterator<Item = usize>>(src: I) -> Self {
        let mut fbs = BVector::with_capacity(0);
        fbs.extend(src);
        fbs
    }
}

impl<'a> BitAnd for &'a BVector {
    type Output = BVector;
    fn bitand(self, other: &BVector) -> BVector {
        let self_len = self.len();
        let other_len = other.len();

        let (short, long) = {
            if self_len <= other_len {
                (&self, &other)
            } else {
                (&other, &self)
            }
        };
        let mut new_bvector = (*short).clone();
        new_bvector.intersect_with(long);

        let res;
        unsafe {
            // TODO: fixedbitset truncates to size of self, while bitmagic doesn't
            res = bitmagic_sys::BM_bvector_set_size(new_bvector.handle, short.len() as u32);
        }
        _check_res(res);
        new_bvector
    }
}

impl<'a> BitAndAssign for BVector {
    fn bitand_assign(&mut self, other: Self) {
        self.intersect_with(&other);
    }
}

impl<'a> BitAndAssign<&Self> for BVector {
    fn bitand_assign(&mut self, other: &Self) {
        self.intersect_with(other);
    }
}

impl<'a> BitOr for &'a BVector {
    type Output = BVector;
    fn bitor(self, other: &BVector) -> BVector {
        let self_len = self.len();
        let other_len = other.len();

        let (short, long) = {
            if self_len <= other_len {
                (&self, &other)
            } else {
                (&other, &self)
            }
        };
        let mut new_bvector = (*short).clone();
        new_bvector.union_with(long);

        new_bvector
    }
}

impl<'a> BitOrAssign for BVector {
    fn bitor_assign(&mut self, other: Self) {
        self.union_with(&other);
    }
}

impl<'a> BitOrAssign<&Self> for BVector {
    fn bitor_assign(&mut self, other: &Self) {
        self.union_with(other);
    }
}

impl<'a> BitXor for &'a BVector {
    type Output = BVector;
    fn bitxor(self, other: &BVector) -> BVector {
        let self_len = self.len();
        let other_len = other.len();

        let (short, long) = {
            if self_len <= other_len {
                (&self, &other)
            } else {
                (&other, &self)
            }
        };
        let mut new_bvector = (*short).clone();
        new_bvector.symmetric_difference_with(long);

        new_bvector
    }
}

impl<'a> BitXorAssign for BVector {
    fn bitxor_assign(&mut self, other: Self) {
        self.symmetric_difference_with(&other);
    }
}

impl<'a> BitXorAssign<&Self> for BVector {
    fn bitxor_assign(&mut self, other: &Self) {
        self.symmetric_difference_with(other);
    }
}

// Taken from https://github.com/bluss/odds/blob/master/src/range.rs.

/// **IndexRange** is implemented by Rust's built-in range types, produced
/// by range syntax like `..`, `a..`, `..b` or `c..d`.
pub trait IndexRange<T = usize> {
    #[inline]
    /// Start index (inclusive)
    fn start(&self) -> Option<T> {
        None
    }
    #[inline]
    /// End index (exclusive)
    fn end(&self) -> Option<T> {
        None
    }
}

impl<T> IndexRange<T> for RangeFull {}

impl<T: Copy> IndexRange<T> for RangeFrom<T> {
    #[inline]
    fn start(&self) -> Option<T> {
        Some(self.start)
    }
}

impl<T: Copy> IndexRange<T> for RangeTo<T> {
    #[inline]
    fn end(&self) -> Option<T> {
        Some(self.end)
    }
}

impl<T: Copy> IndexRange<T> for Range<T> {
    #[inline]
    fn start(&self) -> Option<T> {
        Some(self.start)
    }
    #[inline]
    fn end(&self) -> Option<T> {
        Some(self.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::BVector;

    #[test]
    fn it_works() {
        const N: usize = 50;
        let mut fb = BVector::with_capacity(N);

        for i in 0..(N + 10) {
            assert!(!fb.contains(i));
        }

        fb.insert(10);
        fb.set(11, false);
        fb.set(12, false);
        fb.set(12, true);
        fb.set(N - 1, true);

        assert!(fb.contains(10));
        assert!(!fb.contains(11));
        assert!(fb.contains(12));
        assert!(fb.contains(N - 1));
        for i in 0..N {
            let contain = i == 10 || i == 12 || i == N - 1;
            assert_eq!(contain, fb[i]);
        }

        fb.clear();
    }

    #[test]
    fn grow() {
        let mut fb = BVector::with_capacity(48);
        for i in 0..fb.len() {
            fb.set(i, true);
        }

        let old_len = fb.len();
        fb.grow(72);
        for j in 0..fb.len() {
            assert_eq!(fb.contains(j), j < old_len);
        }
        fb.set(64, true);
        assert!(fb.contains(64));
    }

    #[test]
    fn test_toggle() {
        let mut fb = BVector::with_capacity(16);
        fb.toggle(1);
        fb.put(2);
        fb.toggle(2);
        fb.put(3);
        assert!(fb.contains(1));
        assert!(!fb.contains(2));
        assert!(fb.contains(3));
    }

    #[test]
    fn copy_bit() {
        let mut fb = BVector::with_capacity(48);
        for i in 0..fb.len() {
            fb.set(i, true);
        }
        fb.set(42, false);
        fb.copy_bit(42, 2);
        assert!(!fb.contains(42));
        assert!(!fb.contains(2));
        assert!(fb.contains(1));
        fb.copy_bit(1, 42);
        assert!(fb.contains(42));
        fb.copy_bit(1024, 42);
        assert!(!fb[42]);
    }

    #[test]
    fn count_ones() {
        let mut fb = BVector::with_capacity(100);
        fb.set(11, true);
        fb.set(12, true);
        fb.set(7, true);
        fb.set(35, true);
        fb.set(40, true);
        fb.set(77, true);
        fb.set(95, true);
        fb.set(50, true);
        fb.set(99, true);
        assert_eq!(fb.count_ones(..7), 0);
        assert_eq!(fb.count_ones(..8), 1);
        assert_eq!(fb.count_ones(..11), 1);
        assert_eq!(fb.count_ones(..12), 2);
        assert_eq!(fb.count_ones(..13), 3);
        assert_eq!(fb.count_ones(..35), 3);
        assert_eq!(fb.count_ones(..36), 4);
        assert_eq!(fb.count_ones(..40), 4);
        assert_eq!(fb.count_ones(..41), 5);
        assert_eq!(fb.count_ones(50..), 4);
        assert_eq!(fb.count_ones(70..95), 1);
        assert_eq!(fb.count_ones(70..96), 2);
        assert_eq!(fb.count_ones(70..99), 2);
        assert_eq!(fb.count_ones(..), 9);
        assert_eq!(fb.count_ones(0..100), 9);
        assert_eq!(fb.count_ones(0..0), 0);
        assert_eq!(fb.count_ones(100..100), 0);
        assert_eq!(fb.count_ones(7..), 9);
        assert_eq!(fb.count_ones(8..), 8);
    }

    #[test]
    fn ones() {
        let mut fb = BVector::with_capacity(100);
        fb.set(11, true);
        fb.set(12, true);
        fb.set(7, true);
        fb.set(35, true);
        fb.set(40, true);
        fb.set(77, true);
        fb.set(95, true);
        fb.set(50, true);
        fb.set(99, true);

        let ones: Vec<_> = fb.ones().collect();

        assert_eq!(vec![7, 11, 12, 35, 40, 50, 77, 95, 99], ones);
    }

    #[test]
    fn iter_ones_range() {
        fn test_range(from: usize, to: usize, capa: usize) {
            assert!(to <= capa);
            let mut fb = BVector::with_capacity(capa);
            for i in from..to {
                fb.insert(i);
            }
            let ones: Vec<_> = fb.ones().collect();
            let expected: Vec<_> = (from..to).collect();
            assert_eq!(expected, ones);
        }

        for i in 0..100 {
            test_range(i, 100, 100);
            test_range(0, i, 100);
        }
    }

    #[should_panic]
    #[test]
    fn count_ones_oob() {
        let fb = BVector::with_capacity(100);
        fb.count_ones(90..101);
    }

    #[should_panic]
    #[test]
    fn count_ones_negative_range() {
        let fb = BVector::with_capacity(100);
        fb.count_ones(90..80);
    }

    #[test]
    fn count_ones_panic() {
        for i in 1..128 {
            let fb = BVector::with_capacity(i);
            for j in 0..fb.len() + 1 {
                for k in j..fb.len() + 1 {
                    assert_eq!(fb.count_ones(j..k), 0);
                }
            }
        }
    }

    #[test]
    fn default() {
        let fb = BVector::default();
        assert_eq!(fb.len(), 0);
    }

    #[test]
    fn insert_range() {
        let mut fb = BVector::with_capacity(97);
        fb.insert_range(..3);
        fb.insert_range(9..32);
        fb.insert_range(37..81);
        fb.insert_range(90..);
        for i in 0..97 {
            assert_eq!(
                fb.contains(i),
                i < 3 || (9..32).contains(&i) || (37..81).contains(&i) || 90 <= i
            );
        }
        assert!(!fb.contains(97));
        assert!(!fb.contains(127));
        assert!(!fb.contains(128));
    }

    #[test]
    fn set_range() {
        let mut fb = BVector::with_capacity(48);
        fb.insert_range(..);

        fb.set_range(..32, false);
        fb.set_range(37.., false);
        fb.set_range(5..9, true);
        fb.set_range(40..40, true);

        for i in 0..48 {
            assert_eq!(fb.contains(i), (5..9).contains(&i) || (32..37).contains(&i));
        }
        assert!(!fb.contains(48));
        assert!(!fb.contains(64));
    }

    #[test]
    fn toggle_range() {
        let mut fb = BVector::with_capacity(40);
        fb.insert_range(..10);
        fb.insert_range(34..38);

        fb.toggle_range(5..12);
        fb.toggle_range(30..);

        for i in 0..40 {
            assert_eq!(
                fb.contains(i),
                i < 5 || (10..12).contains(&i) || (30..34).contains(&i) || 38 <= i
            );
        }
        assert!(!fb.contains(40));
        assert!(!fb.contains(64));
    }

    #[test]
    fn bitand_equal_lengths() {
        let len = 109;
        let a_end = 59;
        let b_start = 23;
        let mut a = BVector::with_capacity(len);
        let mut b = BVector::with_capacity(len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a & &b;
        for i in 0..b_start {
            assert!(!ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(ab.contains(i));
        }
        for i in a_end..len {
            assert!(!ab.contains(i));
        }
        assert_eq!(a.len(), ab.len());
    }

    #[test]
    fn bitand_first_smaller() {
        let a_len = 113;
        let b_len = 137;
        let len = std::cmp::min(a_len, b_len);
        let a_end = 97;
        let b_start = 89;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a & &b;
        for i in 0..b_start {
            assert!(!ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(ab.contains(i));
        }
        for i in a_end..len {
            assert!(!ab.contains(i));
        }
        assert_eq!(a.len(), ab.len());
    }

    #[test]
    fn bitand_first_larger() {
        let a_len = 173;
        let b_len = 137;
        let len = std::cmp::min(a_len, b_len);
        let a_end = 107;
        let b_start = 43;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a & &b;
        for i in 0..b_start {
            assert!(!ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(ab.contains(i));
        }
        for i in a_end..len {
            assert!(!ab.contains(i));
        }
        assert_eq!(b.len(), ab.len());
    }

    #[test]
    fn intersection() {
        let len = 109;
        let a_end = 59;
        let b_start = 23;
        let mut a = BVector::with_capacity(len);
        let mut b = BVector::with_capacity(len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);

        let mut ab = a.intersection(&b).collect::<BVector>();

        for i in 0..b_start {
            assert!(!ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(ab.contains(i));
        }
        for i in a_end..len {
            assert!(!ab.contains(i));
        }

        a.intersect_with(&b);
        // intersection + collect produces the same results but with a shorter length.
        ab.grow(a.len());
        assert_eq!(
            ab, a,
            "intersection and intersect_with produce the same results"
        );
    }

    #[test]
    fn union() {
        let a_len = 173;
        let b_len = 137;
        let a_start = 139;
        let b_end = 107;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(a_start.., true);
        b.set_range(..b_end, true);
        let ab = a.union(&b).collect::<BVector>();
        for i in a_start..a_len {
            assert!(ab.contains(i));
        }
        for i in 0..b_end {
            assert!(ab.contains(i));
        }
        for i in b_end..a_start {
            assert!(!ab.contains(i));
        }

        a.union_with(&b);
        assert_eq!(ab, a, "union and union_with produce the same results");
    }

    #[test]
    fn difference() {
        let a_len = 83;
        let b_len = 151;
        let a_start = 0;
        let a_end = 79;
        let b_start = 53;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(a_start..a_end, true);
        b.set_range(b_start..b_len, true);
        let mut a_diff_b = a.difference(&b).collect::<BVector>();
        for i in a_start..b_start {
            assert!(a_diff_b.contains(i));
        }
        for i in b_start..b_len {
            assert!(!a_diff_b.contains(i));
        }

        a.difference_with(&b);
        // difference + collect produces the same results but with a shorter length.
        a_diff_b.grow(a.len());
        assert_eq!(
            a_diff_b, a,
            "difference and difference_with produce the same results"
        );
    }

    #[test]
    fn symmetric_difference() {
        let a_len = 83;
        let b_len = 151;
        let a_start = 47;
        let a_end = 79;
        let b_start = 53;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(a_start..a_end, true);
        b.set_range(b_start..b_len, true);
        let a_sym_diff_b = a.symmetric_difference(&b).collect::<BVector>();
        for i in 0..a_start {
            assert!(!a_sym_diff_b.contains(i));
        }
        for i in a_start..b_start {
            assert!(a_sym_diff_b.contains(i));
        }
        for i in b_start..a_end {
            assert!(!a_sym_diff_b.contains(i));
        }
        for i in a_end..b_len {
            assert!(a_sym_diff_b.contains(i));
        }

        a.symmetric_difference_with(&b);
        assert_eq!(
            a_sym_diff_b, a,
            "symmetric_difference and _with produce the same results"
        );
    }

    #[test]
    fn bitor_equal_lengths() {
        let len = 109;
        let a_start = 17;
        let a_end = 23;
        let b_start = 19;
        let b_end = 59;
        let mut a = BVector::with_capacity(len);
        let mut b = BVector::with_capacity(len);
        a.set_range(a_start..a_end, true);
        b.set_range(b_start..b_end, true);
        let ab = &a | &b;
        for i in 0..a_start {
            assert!(!ab.contains(i));
        }
        for i in a_start..b_end {
            assert!(ab.contains(i));
        }
        for i in b_end..len {
            assert!(!ab.contains(i));
        }
        assert_eq!(ab.len(), len);
    }

    #[test]
    fn bitor_first_smaller() {
        let a_len = 113;
        let b_len = 137;
        let a_end = 89;
        let b_start = 97;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a | &b;
        for i in 0..a_end {
            assert!(ab.contains(i));
        }
        for i in a_end..b_start {
            assert!(!ab.contains(i));
        }
        for i in b_start..b_len {
            assert!(ab.contains(i));
        }
        assert_eq!(b_len, ab.len());
    }

    #[test]
    fn bitor_first_larger() {
        let a_len = 173;
        let b_len = 137;
        let a_start = 139;
        let b_end = 107;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(a_start.., true);
        b.set_range(..b_end, true);
        let ab = &a | &b;
        for i in a_start..a_len {
            assert!(ab.contains(i));
        }
        for i in 0..b_end {
            assert!(ab.contains(i));
        }
        for i in b_end..a_start {
            assert!(!ab.contains(i));
        }
        assert_eq!(a_len, ab.len());
    }

    #[test]
    fn bitxor_equal_lengths() {
        let len = 109;
        let a_end = 59;
        let b_start = 23;
        let mut a = BVector::with_capacity(len);
        let mut b = BVector::with_capacity(len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a ^ &b;
        for i in 0..b_start {
            assert!(ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(!ab.contains(i));
        }
        for i in a_end..len {
            assert!(ab.contains(i));
        }
        assert_eq!(a.len(), ab.len());
    }

    #[test]
    fn bitxor_first_smaller() {
        let a_len = 113;
        let b_len = 137;
        let len = std::cmp::max(a_len, b_len);
        let a_end = 97;
        let b_start = 89;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a ^ &b;
        for i in 0..b_start {
            assert!(ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(!ab.contains(i));
        }
        for i in a_end..len {
            assert!(ab.contains(i));
        }
        assert_eq!(b.len(), ab.len());
    }

    #[test]
    fn bitxor_first_larger() {
        let a_len = 173;
        let b_len = 137;
        let len = std::cmp::max(a_len, b_len);
        let a_end = 107;
        let b_start = 43;
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.set_range(..a_end, true);
        b.set_range(b_start.., true);
        let ab = &a ^ &b;
        for i in 0..b_start {
            assert!(ab.contains(i));
        }
        for i in b_start..a_end {
            assert!(!ab.contains(i));
        }
        for i in a_end..b_len {
            assert!(ab.contains(i));
        }
        for i in b_len..len {
            assert!(!ab.contains(i));
        }
        assert_eq!(a.len(), ab.len());
    }

    #[test]
    fn bitand_assign_shorter() {
        let a_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let b_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
        let a_and_b: Vec<usize> = vec![2, 7, 31, 32];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        a &= b;
        let res = a.ones().collect::<Vec<usize>>();

        assert!(res == a_and_b);
    }

    #[test]
    fn bitand_assign_longer() {
        let a_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
        let b_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let a_and_b: Vec<usize> = vec![2, 7, 31, 32];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        a &= b;
        let res = a.ones().collect::<Vec<usize>>();
        assert!(res == a_and_b, "{:?}", res);
    }

    #[test]
    fn bitor_assign_shorter() {
        let a_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let b_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
        let a_or_b: Vec<usize> = vec![2, 3, 7, 8, 11, 19, 23, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        a |= b;
        let res = a.ones().collect::<Vec<usize>>();
        assert!(res == a_or_b);
    }

    #[test]
    fn bitor_assign_longer() {
        let a_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
        let b_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let a_or_b: Vec<usize> = vec![2, 3, 7, 8, 11, 19, 23, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        a |= b;
        let res = a.ones().collect::<Vec<usize>>();
        assert!(res == a_or_b);
    }

    #[test]
    fn bitxor_assign_shorter() {
        let a_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let b_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
        let a_xor_b: Vec<usize> = vec![3, 8, 11, 19, 23, 37, 41, 43, 47, 71, 73, 101];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        a ^= b;
        let res = a.ones().collect::<Vec<usize>>();
        assert!(res == a_xor_b);
    }

    #[test]
    fn bitxor_assign_longer() {
        let a_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
        let b_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
        let a_xor_b: Vec<usize> = vec![3, 8, 11, 19, 23, 37, 41, 43, 47, 71, 73, 101];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        a ^= b;
        let res = a.ones().collect::<Vec<usize>>();
        assert!(res == a_xor_b);
    }

    #[test]
    fn op_assign_ref() {
        let mut a = BVector::with_capacity(8);
        let b = BVector::with_capacity(8);

        //check that all assign type operators work on references
        a &= &b;
        a |= &b;
        a ^= &b;
    }

    #[test]
    fn subset_superset_shorter() {
        let a_ones: Vec<usize> = vec![7, 31, 32, 63];
        let b_ones: Vec<usize> = vec![2, 7, 19, 31, 32, 37, 41, 43, 47, 63, 73, 101];
        let mut a = a_ones.iter().cloned().collect::<BVector>();
        let b = b_ones.iter().cloned().collect::<BVector>();
        assert!(a.is_subset(&b) && b.is_superset(&a));
        a.insert(14);
        assert!(!a.is_subset(&b) && !b.is_superset(&a));
    }

    #[test]
    fn subset_superset_longer() {
        let a_len = 153;
        let b_len = 75;
        let a_ones: Vec<usize> = vec![7, 31, 32, 63];
        let b_ones: Vec<usize> = vec![2, 7, 19, 31, 32, 37, 41, 43, 47, 63, 73];
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.extend(a_ones.iter().cloned());
        b.extend(b_ones.iter().cloned());
        assert!(a.is_subset(&b) && b.is_superset(&a));
        a.insert(100);
        assert!(!a.is_subset(&b) && !b.is_superset(&a));
    }

    #[test]
    fn is_disjoint_first_shorter() {
        let a_len = 75;
        let b_len = 153;
        let a_ones: Vec<usize> = vec![2, 19, 32, 37, 41, 43, 47, 73];
        let b_ones: Vec<usize> = vec![7, 23, 31, 63, 124];
        let mut a = BVector::with_capacity(a_len);
        let mut b = BVector::with_capacity(b_len);
        a.extend(a_ones.iter().cloned());
        b.extend(b_ones.iter().cloned());
        assert!(a.is_disjoint(&b));
        a.insert(63);
        assert!(!a.is_disjoint(&b));
    }

    #[test]
    fn is_disjoint_first_longer() {
        let a_ones: Vec<usize> = vec![2, 19, 32, 37, 41, 43, 47, 73, 101];
        let b_ones: Vec<usize> = vec![7, 23, 31, 63];
        let a = a_ones.iter().cloned().collect::<BVector>();
        let mut b = b_ones.iter().cloned().collect::<BVector>();
        assert!(a.is_disjoint(&b));
        b.insert(2);
        assert!(!a.is_disjoint(&b));
    }

    #[test]
    fn extend_on_empty() {
        let items: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 167];
        let mut fbs = BVector::with_capacity(0);
        fbs.extend(items.iter().cloned());
        let ones = fbs.ones().collect::<Vec<usize>>();
        assert!(ones == items);
    }

    #[test]
    fn extend() {
        let items: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 167];
        let mut fbs = BVector::with_capacity(168);
        let new: Vec<usize> = vec![7, 37, 67, 137];
        for i in &new {
            fbs.put(*i);
        }

        fbs.extend(items.iter().cloned());

        let ones = fbs.ones().collect::<Vec<usize>>();
        let expected = {
            let mut tmp = items.clone();
            tmp.extend(new);
            tmp.sort_unstable();
            tmp.dedup();
            tmp
        };

        assert!(ones == expected);
    }

    #[test]
    fn from_iterator() {
        let items: Vec<usize> = vec![0, 2, 4, 6, 8];
        let fb = items.iter().cloned().collect::<BVector>();
        for i in items {
            assert!(fb.contains(i));
        }
        for i in &[1usize, 3, 5, 7] {
            assert!(!fb.contains(*i));
        }
        assert_eq!(fb.len(), 9);
    }

    #[test]
    fn from_iterator_ones() {
        let len = 257;
        let mut fb = BVector::with_capacity(len);
        for i in (0..len).filter(|i| i % 7 == 0) {
            fb.put(i);
        }
        fb.put(len - 1);
        let dup = fb.ones().collect::<BVector>();

        assert_eq!(fb.len(), dup.len());
        assert_eq!(
            fb.ones().collect::<Vec<usize>>(),
            dup.ones().collect::<Vec<usize>>()
        );
    }

    #[test]
    fn binary_trait() {
        let items: Vec<usize> = vec![1, 5, 7, 10, 14, 15];
        let fb = items.iter().cloned().collect::<BVector>();

        assert_eq!(format!("{:b}", fb), "0100010100100011");
        assert_eq!(format!("{:#b}", fb), "0b0100010100100011");
    }

    #[test]
    fn display_trait() {
        let len = 8;
        let mut fb = BVector::with_capacity(len);

        fb.put(4);
        fb.put(2);

        assert_eq!(format!("{}", fb), "00101000");
        assert_eq!(format!("{:#}", fb), "0b00101000");
    }
}
