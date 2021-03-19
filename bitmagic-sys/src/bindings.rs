/* automatically generated by rust-bindgen 0.55.1 */

pub const BM_OK: u32 = 0;
pub const BM_ERR_BADALLOC: u32 = 1;
pub const BM_ERR_BADARG: u32 = 2;
pub const BM_ERR_RANGE: u32 = 3;
pub const BM_ERR_CPU: u32 = 4;
pub const BM_ERR_SERIALFORMAT: u32 = 5;
pub const BM_ERR_BAD_VALUE: u32 = 6;
pub const BM_ERR_RANK_SELECT_IDX_MISSING: u32 = 7;
pub const BM_ERR_DETACHED: u32 = 101;
pub const BM_ERR_JVM_NOT_SUPPORTED: u32 = 102;
pub const BM_ERR_JVM_OUT_OF_MEMORY: u32 = 103;
pub const BM_OK_MSG: &'static [u8; 19usize] = b"BM-00: All correct\0";
pub const BM_ERR_BADALLOC_MSG: &'static [u8; 24usize] = b"BM-01: Allocation error\0";
pub const BM_ERR_BADARG_MSG: &'static [u8; 44usize] =
    b"BM-02: Invalid or missing function argument\0";
pub const BM_ERR_RANGE_MSG: &'static [u8; 32usize] = b"BM-03: Incorrect range or index\0";
pub const BM_ERR_CPU_MSG: &'static [u8; 50usize] =
    b"BM-04: Incorrect CPU vectorization (SIMD) version\0";
pub const BM_ERR_SERIALFORMAT_MSG: &'static [u8; 34usize] = b"BM-05: Serialization format error\0";
pub const BM_ERR_BAD_VALUE_MSG: &'static [u8; 17usize] = b"BM-06: Bad value\0";
pub const BM_ERR_DETACHED_MSG: &'static [u8; 42usize] =
    b"BM-101: Current thread no attached to JVM\0";
pub const BM_ERR_JVM_NOT_SUPPORTED_MSG: &'static [u8; 34usize] =
    b"BM-102: JVM version not supported\0";
pub const BM_ERR_JVM_OUT_OF_MEMORY_MSG: &'static [u8; 28usize] = b"BM-103: Out of memory error\0";
pub const BM_UNK_MSG: &'static [u8; 21usize] = b"BM-XX: Unknown error\0";
pub const BM_SIMD_NO: u32 = 0;
pub const BM_SIMD_SSE2: u32 = 1;
pub const BM_SIMD_SSE42: u32 = 2;
pub const BM_SIMD_AVX2: u32 = 5;
pub const BM_TRUE: u32 = 1;
pub const BM_FALSE: u32 = 0;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::core::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::core::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BM_bvector_statistics {
    pub bit_blocks: usize,
    pub gap_blocks: usize,
    pub max_serialize_mem: usize,
    pub memory_used: usize,
}
#[test]
fn bindgen_test_layout_BM_bvector_statistics() {
    assert_eq!(
        ::core::mem::size_of::<BM_bvector_statistics>(),
        32usize,
        concat!("Size of: ", stringify!(BM_bvector_statistics))
    );
    assert_eq!(
        ::core::mem::align_of::<BM_bvector_statistics>(),
        8usize,
        concat!("Alignment of ", stringify!(BM_bvector_statistics))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BM_bvector_statistics>())).bit_blocks as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BM_bvector_statistics),
            "::",
            stringify!(bit_blocks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BM_bvector_statistics>())).gap_blocks as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BM_bvector_statistics),
            "::",
            stringify!(gap_blocks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BM_bvector_statistics>())).max_serialize_mem as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(BM_bvector_statistics),
            "::",
            stringify!(max_serialize_mem)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BM_bvector_statistics>())).memory_used as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(BM_bvector_statistics),
            "::",
            stringify!(memory_used)
        )
    );
}
extern "C" {
    pub fn BM_init(arg1: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "return copyright info string and version information."]
    pub fn BM_version(
        major: *mut ::std::os::raw::c_int,
        minor: *mut ::std::os::raw::c_int,
        patch: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "return SIMD version used to build binaries"]
    #[doc = "one of BM_SIMD_* defines"]
    pub fn BM_simd_version() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "return error message by code"]
    pub fn BM_error_msg(errcode: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn BM_bvector_construct(
        h: *mut *mut ::core::ffi::c_void,
        bv_max: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_init(h: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_construct_copy(
        h: *mut *mut ::core::ffi::c_void,
        hfrom: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_free(h: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_get_size(
        h: *mut ::core::ffi::c_void,
        psize: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set_size(
        h: *mut ::core::ffi::c_void,
        new_size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_swap(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set_bit(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
        val: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set_bits(
        h: *mut ::core::ffi::c_void,
        idx: *mut ::std::os::raw::c_uint,
        idx_size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set_bit_no_check(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set_bit_conditional(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
        val: ::std::os::raw::c_int,
        condition: ::std::os::raw::c_int,
        pchanged: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_flip_bit(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_inc_bit(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
        carry_over: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set(h: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_set_range(
        h: *mut ::core::ffi::c_void,
        left: ::std::os::raw::c_uint,
        right: ::std::os::raw::c_uint,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_invert(h: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_clear(
        h: *mut ::core::ffi::c_void,
        free_mem: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_extract_next(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
        pnext: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_get_bit(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
        pval: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_count(
        h: *mut ::core::ffi::c_void,
        pcount: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_count_range(
        h: *mut ::core::ffi::c_void,
        left: ::std::os::raw::c_uint,
        right: ::std::os::raw::c_uint,
        pcount: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_any(
        h: *mut ::core::ffi::c_void,
        pval: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_find(
        h: *mut ::core::ffi::c_void,
        from: ::std::os::raw::c_uint,
        ppos: *mut ::std::os::raw::c_uint,
        pfound: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_find_reverse(
        h: *mut ::core::ffi::c_void,
        ppos: *mut ::std::os::raw::c_uint,
        pfound: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_find_rank(
        h: *mut ::core::ffi::c_void,
        rank: ::std::os::raw::c_uint,
        from: ::std::os::raw::c_uint,
        pidx: *mut ::std::os::raw::c_uint,
        pfound: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_get_first(
        h: *mut ::core::ffi::c_void,
        pi: *mut ::std::os::raw::c_uint,
        pfound: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_get_next(
        h: *mut ::core::ffi::c_void,
        i: ::std::os::raw::c_uint,
        pnext: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_compare(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pres: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_find_first_mismatch(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pi: *mut ::std::os::raw::c_uint,
        pfound: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_optimize(
        h: *mut ::core::ffi::c_void,
        opt_mode: ::std::os::raw::c_int,
        pstat: *mut BM_bvector_statistics,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_calc_stat(
        h: *mut ::core::ffi::c_void,
        pstat: *mut BM_bvector_statistics,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_operation(
        hdst: *mut ::core::ffi::c_void,
        hsrc: *mut ::core::ffi::c_void,
        opcode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_AND(
        hdst: *mut ::core::ffi::c_void,
        hsrc: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_AND_2sc(
        hdst: *mut ::core::ffi::c_void,
        hsrc1: *mut ::core::ffi::c_void,
        hsrc2: *mut ::core::ffi::c_void,
        compress: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_OR(
        hdst: *mut ::core::ffi::c_void,
        hsrc: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_OR_2sc(
        hdst: *mut ::core::ffi::c_void,
        hsrc1: *mut ::core::ffi::c_void,
        hsrc2: *mut ::core::ffi::c_void,
        compress: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_SUB(
        hdst: *mut ::core::ffi::c_void,
        hsrc: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_SUB_2sc(
        hdst: *mut ::core::ffi::c_void,
        hsrc1: *mut ::core::ffi::c_void,
        hsrc2: *mut ::core::ffi::c_void,
        compress: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_XOR(
        hdst: *mut ::core::ffi::c_void,
        hsrc: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_XOR_2sc(
        hdst: *mut ::core::ffi::c_void,
        hsrc1: *mut ::core::ffi::c_void,
        hsrc2: *mut ::core::ffi::c_void,
        compress: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_merge(
        hdst: *mut ::core::ffi::c_void,
        hsrc: *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_rshift1(hdst: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_OR_arr(
        hdst: *mut ::core::ffi::c_void,
        arr_begin: *const ::std::os::raw::c_uint,
        arr_end: *const ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_XOR_arr(
        hdst: *mut ::core::ffi::c_void,
        arr_begin: *const ::std::os::raw::c_uint,
        arr_end: *const ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_SUB_arr(
        hdst: *mut ::core::ffi::c_void,
        arr_begin: *const ::std::os::raw::c_uint,
        arr_end: *const ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_AND_arr(
        hdst: *mut ::core::ffi::c_void,
        arr_begin: *const ::std::os::raw::c_uint,
        arr_end: *const ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_combine_AND_arr_sorted(
        hdst: *mut ::core::ffi::c_void,
        arr_begin: *const ::std::os::raw::c_uint,
        arr_end: *const ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_construct(
        h: *mut ::core::ffi::c_void,
        peh: *mut *mut ::core::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_construct_from(
        h: *mut ::core::ffi::c_void,
        peh: *mut *mut ::core::ffi::c_void,
        pos: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_free(eh: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_is_valid(
        eh: *mut ::core::ffi::c_void,
        pvalid: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_get_value(
        eh: *mut ::core::ffi::c_void,
        pvalue: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_next(
        eh: *mut ::core::ffi::c_void,
        pvalid: *mut ::std::os::raw::c_int,
        pvalue: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_enumerator_goto(
        eh: *mut ::core::ffi::c_void,
        pos: ::std::os::raw::c_uint,
        pvalid: *mut ::std::os::raw::c_int,
        pvalue: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_serialize(
        h: *mut ::core::ffi::c_void,
        buf: *mut ::std::os::raw::c_char,
        buf_size: usize,
        pblob_size: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_deserialize(
        h: *mut ::core::ffi::c_void,
        buf: *const ::std::os::raw::c_char,
        buf_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_count_AND(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pcount: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_any_AND(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pany: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_count_XOR(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pcount: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_any_XOR(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pany: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_count_SUB(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pcount: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_any_SUB(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pany: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_count_OR(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pcount: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn BM_bvector_any_OR(
        h1: *mut ::core::ffi::c_void,
        h2: *mut ::core::ffi::c_void,
        pany: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
