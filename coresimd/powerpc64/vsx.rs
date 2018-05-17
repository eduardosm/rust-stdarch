//! PowerPC Vectir Scalar eXtensions (VSX) intrinsics.
//!
//! The references are: [POWER ISA v2.07B (for POWER8 & POWER8 with NVIDIA
//! NVlink)] and [POWER ISA v3.0B (for POWER9)].
//!
//! [POWER ISA v2.07B (for POWER8 & POWER8 with NVIDIA NVlink)]: https://ibm.box.com/s/jd5w15gz301s5b5dt375mshpq9c3lh4u
//! [POWER ISA v3.0B (for POWER9)]: https://ibm.box.com/s/1hzcwkwf8rbju5h9iyf44wm94amnlcrv

#![allow(non_camel_case_types)]

use coresimd::powerpc::*;
use coresimd::simd::*;
use coresimd::simd_llvm::*;

#[cfg(test)]
use stdsimd_test::assert_instr;

use mem;

types! {
    // pub struct vector_Float16 = f16x8;
    /// PowerPC-specific 128-bit wide vector of two packed `i64`
    pub struct vector_signed_long(i64, i64);
    /// PowerPC-specific 128-bit wide vector of two packed `u64`
    pub struct vector_unsigned_long(u64, u64);
    /// PowerPC-specific 128-bit wide vector mask of two elements
    pub struct vector_bool_long(i64, i64);
    /// PowerPC-specific 128-bit wide vector of two packed `f64`
    pub struct vector_double(f64, f64);
    // pub struct vector_signed_long_long = vector_signed_long;
    // pub struct vector_unsigned_long_long = vector_unsigned_long;
    // pub struct vector_bool_long_long = vector_bool_long;
    // pub struct vector_signed___int128 = i128x1;
    // pub struct vector_unsigned___int128 = i128x1;
}

impl_from_bits_!(
    vector_signed_long: u64x2,
    i64x2,
    f64x2,
    m64x2,
    u32x4,
    i32x4,
    f32x4,
    m32x4,
    u16x8,
    i16x8,
    m16x8,
    u8x16,
    i8x16,
    m8x16,
    vector_unsigned_char,
    vector_bool_char,
    vector_signed_short,
    vector_unsigned_short,
    vector_bool_short,
    vector_signed_int,
    vector_unsigned_int,
    vector_float,
    vector_bool_int,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);
impl_from_bits_!(
    i64x2:
    vector_signed_char,
    vector_unsigned_char,
    vector_bool_char,
    vector_signed_short,
    vector_unsigned_short,
    vector_bool_short,
    vector_signed_int,
    vector_unsigned_int,
    vector_float,
    vector_bool_int,
    vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_unsigned_long: u64x2,
    i64x2,
    f64x2,
    m64x2,
    u32x4,
    i32x4,
    f32x4,
    m32x4,
    u16x8,
    i16x8,
    m16x8,
    u8x16,
    i8x16,
    m8x16,
    vector_unsigned_char,
    vector_bool_char,
    vector_signed_short,
    vector_unsigned_short,
    vector_bool_short,
    vector_signed_int,
    vector_unsigned_int,
    vector_float,
    vector_bool_int,
    vector_signed_long,
    vector_bool_long,
    vector_double
);
impl_from_bits_!(
    u64x2:
    vector_signed_char,
    vector_unsigned_char,
    vector_bool_char,
    vector_signed_short,
    vector_unsigned_short,
    vector_bool_short,
    vector_signed_int,
    vector_unsigned_int,
    vector_float,
    vector_bool_int,
    vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_double: u64x2,
    i64x2,
    f64x2,
    m64x2,
    u32x4,
    i32x4,
    f32x4,
    m32x4,
    u16x8,
    i16x8,
    m16x8,
    u8x16,
    i8x16,
    m8x16,
    vector_unsigned_char,
    vector_bool_char,
    vector_signed_short,
    vector_unsigned_short,
    vector_bool_short,
    vector_signed_int,
    vector_unsigned_int,
    vector_float,
    vector_bool_int,
    vector_signed_long,
    vector_unsigned_long,
    vector_bool_long
);
impl_from_bits_!(
    f64x2:
    vector_signed_char,
    vector_unsigned_char,
    vector_bool_char,
    vector_signed_short,
    vector_unsigned_short,
    vector_bool_short,
    vector_signed_int,
    vector_unsigned_int,
    vector_float,
    vector_bool_int,
    vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(vector_bool_long: m64x2);
impl_from_bits_!(m64x2: vector_bool_long);
impl_from_bits_!(m32x4: vector_bool_long);
impl_from_bits_!(m16x8: vector_bool_long);
impl_from_bits_!(m8x16: vector_bool_long);
impl_from_bits_!(vector_bool_char: vector_bool_long);
impl_from_bits_!(vector_bool_short: vector_bool_long);
impl_from_bits_!(vector_bool_int: vector_bool_long);

impl_from_bits_!(
    vector_signed_char: vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_unsigned_char: vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_signed_short: vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_unsigned_short: vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_signed_int: vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

impl_from_bits_!(
    vector_unsigned_int: vector_signed_long,
    vector_unsigned_long,
    vector_bool_long,
    vector_double
);

mod sealed {

    use super::*;

    pub trait VectorPermDI {
        unsafe fn vec_xxpermdi(self, b: Self, dm: u8) -> Self;
    }

    #[inline]
    #[target_feature(enable = "vsx")]
    #[cfg_attr(test, assert_instr(xxpermdi, dm = 0x0))]
    unsafe fn xxpermdi(a: i64x2, b: i64x2, dm: u8) -> i64x2 {
        match dm & 0b11 {
            0 => simd_shuffle2(a, b, [0b00, 0b10]),
            1 => simd_shuffle2(a, b, [0b01, 0b10]),
            2 => simd_shuffle2(a, b, [0b00, 0b11]),
            _ => simd_shuffle2(a, b, [0b01, 0b11]),
        }
    }

    macro_rules! vec_xxpermdi {
        {$impl: ident} => {
            impl VectorPermDI for $impl {
                #[inline]
                #[target_feature(enable = "vsx")]
                unsafe fn vec_xxpermdi(self, b: Self, dm: u8) -> Self {
                    mem::transmute(xxpermdi(mem::transmute(self), mem::transmute(b), dm))
                }
            }
        }
    }

    vec_xxpermdi! { vector_unsigned_long }
    vec_xxpermdi! { vector_signed_long }
    vec_xxpermdi! { vector_bool_long }
    vec_xxpermdi! { vector_double }
}

/// Vector permute.
#[inline]
#[target_feature(enable = "vsx")]
#[rustc_args_required_const(2)]
pub unsafe fn vec_xxpermdi<T>(a: T, b: T, dm: u8) -> T
where
    T: sealed::VectorPermDI,
{
    a.vec_xxpermdi(b, dm)
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "powerpc")]
    use coresimd::arch::powerpc::*;

    #[cfg(target_arch = "powerpc64")]
    use coresimd::arch::powerpc64::*;

    use simd::*;
    use stdsimd_test::simd_test;

    #[simd_test(enable = "vsx")]
    unsafe fn vec_xxpermdi_u62x2() {
        let a: vector_unsigned_long = u64x2::new(0, 1).into_bits();
        let b = u64x2::new(2, 3).into_bits();

        assert_eq!(u64x2::new(0, 2), vec_xxpermdi(a, b, 0).into_bits());
        assert_eq!(u64x2::new(1, 2), vec_xxpermdi(a, b, 1).into_bits());
        assert_eq!(u64x2::new(0, 3), vec_xxpermdi(a, b, 2).into_bits());
        assert_eq!(u64x2::new(1, 3), vec_xxpermdi(a, b, 3).into_bits());
    }
}