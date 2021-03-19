#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::redundant_static_lifetimes)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_void;
    use std::ptr;

    #[test]
    fn construction() {
        unsafe {
            BM_init(ptr::null_mut());

            let mut res = 0;

            let mut h = ptr::null_mut();
            res = BM_bvector_construct(&mut h, 100);
            // TODO: check res
            // BMERR_CHECK(res, "BM_bvector_construct()");

            BM_bvector_free(h);
        }
    }

    #[test]
    fn resize() {
        unsafe {
            BM_init(ptr::null_mut());

            let mut res = 0;
            let size1 = 100000;
            let size2 = 100000;
            let mut size = 0;

            let mut h = ptr::null_mut();
            res = BM_bvector_construct(&mut h, size1);
            // TODO: check res

            res = BM_bvector_get_size(h, &mut size);
            // TODO: check res
            assert_eq!(size, size1);

            res = BM_bvector_set_size(h, size2);
            // TODO: check res
            res = BM_bvector_get_size(h, &mut size);
            // TODO: check res
            assert_eq!(size, size2);

            BM_bvector_free(h);
        }
    }
}
