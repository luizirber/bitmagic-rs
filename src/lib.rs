//! Rust bindings for the BitMagic C++ Library
//! For more information please visit: http://bitmagic.io.

#![deny(missing_docs)]

use std::io::{Read, Write};
use std::os::raw::c_void;
use std::ptr;
use std::sync::Once;

mod fixedbitset_api;

static START: Once = Once::new();
/// Initialize libbm runtime before use
fn init_lib() {
    START.call_once(|| {
        unsafe {
            // TODO: check result, panic on error?
            let res = bitmagic_sys::BM_init(ptr::null_mut());
            // TODO: can be BM_OK or BM_ERR_CPU
        };
    });
}

/// A bitvector
pub struct BVector {
    handle: *mut c_void,
}

impl BVector {
    /// Serialize bit vector
    pub fn serialize<W>(&self, mut wtr: W) -> Result<(), Box<dyn std::error::Error>>
    where
        W: Write,
    {
        let mut res = 0;
        let mut bv_stat = bitmagic_sys::BM_bvector_statistics {
            bit_blocks: 0,
            gap_blocks: 0,
            max_serialize_mem: 0,
            memory_used: 0,
        };

        unsafe {
            res = bitmagic_sys::BM_bvector_optimize(self.handle, 3, &mut bv_stat);
            // TODO: check res
        }

        let mut buf = vec![0u8; bv_stat.max_serialize_mem as usize];
        let mut blob_size = 0;
        unsafe {
            res = bitmagic_sys::BM_bvector_serialize(
                self.handle,
                buf.as_mut_ptr() as *mut i8,
                buf.len() as u64,
                &mut blob_size,
            );
            // TODO: check res
        }

        if blob_size == 0 || blob_size > bv_stat.max_serialize_mem {
            todo!("throw error")
        }

        wtr.write_all(buf.as_slice())?;

        Ok(())
    }

    /// Deserialize bit vector
    pub fn deserialize<R>(mut rdr: R) -> Result<Self, Box<dyn std::error::Error>>
    where
        R: Read,
    {
        let mut res = 0;
        let mut buf = vec![];
        rdr.read_to_end(&mut buf)?;

        let bnew = BVector::with_capacity(1);

        unsafe {
            res = bitmagic_sys::BM_bvector_deserialize(
                bnew.handle,
                buf.as_mut_ptr() as *mut i8,
                buf.len() as u64,
            );
            // TODO: check res
        }

        Ok(bnew)
    }

    /// Size of the intersection of two `BVector`s.
    ///
    /// Equivalent to the population count of AND of two bit vectors
    pub fn intersection_count(&self, other: &BVector) -> usize {
        let mut res = 0;
        let mut pcount = 0;

        unsafe {
            res = bitmagic_sys::BM_bvector_count_AND(self.handle, other.handle, &mut pcount);
            // TODO: check res
        }

        pcount as usize
    }

    /// Size of the union of two `BVector`s.
    ///
    /// Equivalent to the population count of OR of two bit vectors
    pub fn union_count(&self, other: &BVector) -> usize {
        let mut res = 0;
        let mut pcount = 0;

        unsafe {
            res = bitmagic_sys::BM_bvector_count_OR(self.handle, other.handle, &mut pcount);
            // TODO: check res
        }

        pcount as usize
    }

    /// Size of the difference of two `BVector`s.
    ///
    /// Equivalent to the population count of SUB of two bit vectors
    pub fn difference_count(&self, other: &BVector) -> usize {
        let mut res = 0;
        let mut pcount = 0;

        unsafe {
            res = bitmagic_sys::BM_bvector_count_SUB(self.handle, other.handle, &mut pcount);
            // TODO: check res
        }

        pcount as usize
    }

    /// Size of the symmetric difference of two `BVector`s.
    ///
    /// Equivalent to the population count of XOR of two bit vectors
    pub fn symmetric_difference_count(&self, other: &BVector) -> usize {
        let mut res = 0;
        let mut pcount = 0;

        unsafe {
            res = bitmagic_sys::BM_bvector_count_XOR(self.handle, other.handle, &mut pcount);
            // TODO: check res
        }

        pcount as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::BVector;

    #[test]
    fn serde() {
        let mut bv = BVector::with_capacity(100);
        bv.set_range(10..20, true);
        bv.set_range(50..70, true);

        let mut wtr = vec![];
        bv.serialize(&mut wtr).unwrap();

        let new_bv = BVector::deserialize(wtr.as_slice()).unwrap();

        assert_eq!(new_bv, bv);

        for i in 10..20 {
            assert!(new_bv.contains(i));
        }

        for i in 50..70 {
            assert!(new_bv.contains(i));
        }
    }
}
