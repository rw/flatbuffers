use std::marker::PhantomData;
use std::mem::size_of;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

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
        let sz = size_of::<T>();
        debug_assert!(sz > 0);
        T::follow(self.0, self.1 as usize + SIZE_UOFFSET + sz * idx)
    }
}

impl<'a, T: GeneratedStruct + 'a> Vector<'a, T> {
    pub fn as_slice(self) -> &'a [T] {
        <SliceOfGeneratedStruct<T>>::follow(self.0, self.1)
    }
}

pub trait SafeSliceAccess {}
impl<'a, T: SafeSliceAccess + 'a> Vector<'a, T> {
    pub fn safe_slice(self) -> &'a [T] {
        let buf = self.0;
        let loc = self.1;
        let sz = size_of::<T>();
        debug_assert!(sz > 0);
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { from_raw_parts(ptr, len) };
        s
    }
}

pub fn follow_cast_ref<'a, T: Sized + 'a>(buf: &'a [u8], loc: usize) -> &'a T {
    let sz = size_of::<T>();
    let buf = &buf[loc..loc + sz];
    let ptr = buf.as_ptr() as *const T;
    unsafe { &*ptr }
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

impl<'a> Follow<'a> for &'a str {
    type Inner = &'a str;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let slice = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len];
        let s = unsafe { from_utf8_unchecked(slice) };
        s
    }
}

/// Implement direct slice access to structs (they are endian-safe because we
/// use accessors to get their elements).
impl<'a, T: GeneratedStruct + 'a> Follow<'a> for SliceOfGeneratedStruct<T> {
    type Inner = &'a [T];
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let sz = size_of::<T>();
        assert!(sz > 0);
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { from_raw_parts(ptr, len) };
        s
    }
}

fn follow_slice_helper<T>(buf: &[u8], loc: usize) -> &[T] {
    let sz = size_of::<T>();
    debug_assert!(sz > 0);
    let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
    let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
    let ptr = data_buf.as_ptr() as *const T;
    let s: &[T] = unsafe { from_raw_parts(ptr, len) };
    s
}

/// Implement direct slice access if the host is little-endian.
#[cfg(target_endian = "little")]
impl<'a, T: EndianScalar> Follow<'a> for &'a [T] {
    type Inner = &'a [T];
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        follow_slice_helper::<T>(buf, loc)
    }
}

/// Implement Follow for all possible Vectors that have Follow-able elements.
impl<'a, T: Follow<'a> + 'a> Follow<'a> for Vector<'a, T> {
    type Inner = Vector<'a, T>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Vector::new(buf, loc)
    }
}

