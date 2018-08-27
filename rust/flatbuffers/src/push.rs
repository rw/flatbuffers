use std::mem::size_of;
use std::slice::from_raw_parts;

pub use primitives::*;
pub use endian_scalar::{EndianScalar, read_scalar, emplace_scalar};
pub use vector::*;

pub trait Push: Sized {
    type Output;
    fn push(&self, dst: &mut [u8], _rest: &[u8]);
    fn size(&self) -> usize {
        size_of::<Self>()
    }
    fn alignment(&self) -> usize {
        self.size()
    }
}


pub fn pushable_method_struct_push<T: Sized>(x: &T, dst: &mut [u8], _rest: &[u8]) {
    let sz = size_of::<T>();
    debug_assert_eq!(sz, dst.len());
    let src = unsafe {
        from_raw_parts(x as *const T as *const u8, sz)
    };
    dst.copy_from_slice(src);
}

impl<'b> Push for &'b [u8] {
    type Output = Vector<'b, u8>;
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let l = self.len() as UOffsetT;
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l);
        dst[SIZE_UOFFSET..].copy_from_slice(self);
    }
    fn size(&self) -> usize {
        self.len() + SIZE_UOFFSET
    }
    fn alignment(&self) -> usize {
        SIZE_UOFFSET
    }
}

pub struct ZeroTerminatedByteSlice<'a>(&'a [u8]);

impl<'a> ZeroTerminatedByteSlice<'a> {
    #[inline(always)]
    pub fn new(buf: &'a [u8]) -> Self {
        ZeroTerminatedByteSlice { 0: buf }
    }

    #[inline(always)]
    pub fn data(&'a self) -> &'a [u8] {
        self.0
    }
}
impl<'b> Push for ZeroTerminatedByteSlice<'b> {
    type Output = Vector<'b, u8>;

    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let l = self.data().len();
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l as UOffsetT);
        dst[SIZE_UOFFSET..SIZE_UOFFSET+l].copy_from_slice(self.data());
    }
    fn size(&self) -> usize {
        SIZE_UOFFSET + self.0.len() + 1
    }
    fn alignment(&self) -> usize {
        SIZE_UOFFSET
    }
}

macro_rules! impl_pushable_method_for_endian_scalar {
    ($ty:ident) => (
        impl Push for $ty {
            type Output = $ty;

            #[inline(always)]
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

impl<T> Push for Offset<T> {
    type Output = ForwardsUOffset<T>;

    #[inline(always)]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        let n = (SIZE_UOFFSET + rest.len() - self.value() as usize) as UOffsetT;
        emplace_scalar::<UOffsetT>(dst, n);
    }
}
impl<T> Push for ForwardsUOffset<T> {
    type Output = Self;

    #[inline(always)]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        self.value().push(dst, rest);
    }
}
impl<T> Push for ForwardsVOffset<T> {
    type Output = Self;

    #[inline(always)]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        self.value().push(dst, rest);
    }
}
impl<T> Push for BackwardsSOffset<T> {
    type Output = Self;

    #[inline(always)]
    fn push(&self, dst: &mut [u8], rest: &[u8]) {
        self.value().push(dst, rest);
    }
}

