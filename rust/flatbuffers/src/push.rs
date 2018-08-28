use std::mem::size_of;

use endian_scalar::emplace_scalar;
use primitives::*;
use vector::Vector;

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

impl<'b> Push for &'b [u8] {
    type Output = Vector<'b, u8>;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let l = self.len() as UOffsetT;
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l);
        dst[SIZE_UOFFSET..].copy_from_slice(self);
    }

    #[inline]
    fn size(&self) -> usize {
        SIZE_UOFFSET + self.len()
    }

    #[inline]
    fn alignment(&self) -> usize {
        SIZE_UOFFSET
    }
}

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

impl<'b> Push for ZeroTerminatedByteSlice<'b> {
    type Output = Vector<'b, u8>;

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

macro_rules! impl_pushable_method_for_endian_scalar {
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

impl_pushable_method_for_endian_scalar!(bool);
impl_pushable_method_for_endian_scalar!(u8);
impl_pushable_method_for_endian_scalar!(i8);
impl_pushable_method_for_endian_scalar!(u16);
impl_pushable_method_for_endian_scalar!(i16);
impl_pushable_method_for_endian_scalar!(u32);
impl_pushable_method_for_endian_scalar!(i32);
impl_pushable_method_for_endian_scalar!(u64);
impl_pushable_method_for_endian_scalar!(i64);
impl_pushable_method_for_endian_scalar!(f32);
impl_pushable_method_for_endian_scalar!(f64);

impl<T> Push for WIPOffset<T> {
    type Output = ForwardsUOffset<T>;

    #[inline]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        let n = (SIZE_UOFFSET + rest.len() - self.value() as usize) as UOffsetT;
        emplace_scalar::<UOffsetT>(dst, n);
    }
}
impl<T> Push for ForwardsUOffset<T> {
    type Output = Self;

    #[inline]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        self.value().push(dst, rest);
    }
}
impl<T> Push for ForwardsVOffset<T> {
    type Output = Self;

    #[inline]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        self.value().push(dst, rest);
    }
}
impl<T> Push for BackwardsSOffset<T> {
    type Output = Self;

    #[inline]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        self.value().push(dst, rest);
    }
}

