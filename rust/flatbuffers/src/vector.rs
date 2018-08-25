use std::marker::PhantomData;
use follow::Follow;
use endian_scalar::*;
use primitives::*;

#[derive(Debug)]
pub struct Vector<'a, T: 'a>(&'a [u8], usize, PhantomData<T>);

impl<'a, T: 'a> Vector<'a, T> {
    pub fn new(buf: &'a [u8], loc: usize) -> Self {
        Vector {
            0: buf,
            1: loc,
            2: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        read_scalar::<UOffsetT>(&self.0[self.1 as usize..]) as usize
    }
}

impl<'a, T: Follow<'a> + 'a> Vector<'a, T> {
    pub fn get(&self, idx: usize) -> T::Inner {
        debug_assert!(idx < read_scalar::<u32>(&self.0[self.1 as usize..]) as usize);
        let sz = std::mem::size_of::<T>();
        debug_assert!(sz > 0);
        T::follow(self.0, self.1 as usize + SIZE_UOFFSET + sz * idx)
    }
}

impl<'a, T: GeneratedStruct + 'a> Vector<'a, T> {
    pub fn as_slice(self) -> &'a [T] {
        <SliceOfGeneratedStruct<T>>::follow(self.0, self.1)
    }
}



impl<'a> Vector<'a, bool> { pub fn as_slice(self) -> &'a [bool] { <&'a [bool]>::follow(self.0, self.1) } }
impl<'a> Vector<'a, u8> { pub fn as_slice(self) -> &'a [u8] { <&'a [u8]>::follow(self.0, self.1) } }
impl<'a> Vector<'a, i8> { pub fn as_slice(self) -> &'a [i8] { <&'a [i8]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, u16> { pub fn as_slice(self) -> &'a [u16] { <&'a [u16]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, i16> { pub fn as_slice(self) -> &'a [i16] { <&'a [i16]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, u32> { pub fn as_slice(self) -> &'a [u32] { <&'a [u32]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, i32> { pub fn as_slice(self) -> &'a [i32] { <&'a [i32]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, u64> { pub fn as_slice(self) -> &'a [u64] { <&'a [u64]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, i64> { pub fn as_slice(self) -> &'a [i64] { <&'a [i64]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, f32> { pub fn as_slice(self) -> &'a [f32] { <&'a [f32]>::follow(self.0, self.1) } }
#[cfg(target_endian = "little")]
impl<'a> Vector<'a, f64> { pub fn as_slice(self) -> &'a [f64] { <&'a [f64]>::follow(self.0, self.1) } }
