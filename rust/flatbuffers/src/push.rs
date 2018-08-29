use std::cmp::max;
use std::mem::size_of;
use std::slice::from_raw_parts;

use endian_scalar::emplace_scalar;
use primitives::*;
use vector::{SafeSliceAccess, Vector};

/// Trait to abstract over functionality needed to write values. Used in
/// FlatBufferBuilder and implemented for generated types.
pub trait Push: Sized {
    type Output;
    fn push(&self, dst: &mut [u8], _rest: &[u8]);

    #[inline]
    fn size(&self) -> usize {
        size_of::<Self>()
    }

    #[inline]
    fn alignment(&self) -> usize {
        self.size()
    }
}

//impl<'b> Push for &'b [u8] {
//    type Output = Vector<'b, u8>;
//
//    #[inline]
//    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
//        let l = self.len() as UOffsetT;
//        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l);
//        dst[SIZE_UOFFSET..].copy_from_slice(self);
//    }
//
//    #[inline]
//    fn size(&self) -> usize {
//        SIZE_UOFFSET + self.len()
//    }
//
//    #[inline]
//    fn alignment(&self) -> usize {
//        SIZE_UOFFSET
//    }
//}

impl<'b> Push for &'b str {
    type Output = Vector<'b, u8>;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let l = self.len();
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l as UOffsetT);
        dst[SIZE_UOFFSET..SIZE_UOFFSET+l].copy_from_slice(self.as_bytes());
    }

    #[inline]
    fn size(&self) -> usize {
        SIZE_UOFFSET + self.len() + 1
    }

    #[inline]
    fn alignment(&self) -> usize {
        SIZE_UOFFSET
    }
}

/// Push-able wrapper for slices of types that implement SafeSliceAccess.
impl<'a, T: SafeSliceAccess + Sized> Push for &'a [T] {
    type Output = Vector<'a, u8>;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let elem_sz = size_of::<T>();
        let data = {
            let ptr = self.as_ptr() as *const T as *const u8;
            unsafe {
                from_raw_parts(ptr, self.len() * elem_sz)
            }
        };
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], self.len() as UOffsetT);
        dst[SIZE_UOFFSET..SIZE_UOFFSET+data.len()].copy_from_slice(data);
    }

    #[inline]
    fn size(&self) -> usize {
        SIZE_UOFFSET + self.len() * size_of::<T>()
    }

    #[inline]
    fn alignment(&self) -> usize {
        max(SIZE_UOFFSET, size_of::<T>())
    }
}


/// Push-able wrapper for byte slices that need a zero-terminator written
/// after them.
pub struct ZeroTerminatedByteSlice<'a>(&'a [u8]);

impl<'a> ZeroTerminatedByteSlice<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> Self {
        ZeroTerminatedByteSlice { 0: buf }
    }

    #[inline]
    pub fn data(&'a self) -> &'a [u8] {
        self.0
    }
}

impl<'a> Push for ZeroTerminatedByteSlice<'a> {
    type Output = Vector<'a, u8>;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let l = self.data().len();
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l as UOffsetT);
        dst[SIZE_UOFFSET..SIZE_UOFFSET+l].copy_from_slice(self.data());
    }

    #[inline]
    fn size(&self) -> usize {
        SIZE_UOFFSET + self.0.len() + 1
    }

    #[inline]
    fn alignment(&self) -> usize {
        SIZE_UOFFSET
    }
}

/// Macro to implement Push for EndianScalar types.
macro_rules! impl_push_for_endian_scalar {
    ($ty:ident) => (
        impl Push for $ty {
            type Output = $ty;

            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                emplace_scalar::<$ty>(dst, *self);
            }
        }
    )
}

impl_push_for_endian_scalar!(bool);
impl_push_for_endian_scalar!(u8);
impl_push_for_endian_scalar!(i8);
impl_push_for_endian_scalar!(u16);
impl_push_for_endian_scalar!(i16);
impl_push_for_endian_scalar!(u32);
impl_push_for_endian_scalar!(i32);
impl_push_for_endian_scalar!(u64);
impl_push_for_endian_scalar!(i64);
impl_push_for_endian_scalar!(f32);
impl_push_for_endian_scalar!(f64);
