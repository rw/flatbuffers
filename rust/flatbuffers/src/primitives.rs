use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Deref;

use endian_scalar::*;

pub const FLATBUFFERS_MAX_BUFFER_SIZE: usize = (2u64 << 31) as usize;

pub const FILE_IDENTIFIER_LENGTH: usize = 4;

pub const VTABLE_METADATA_FIELDS: usize = 2;

pub const SIZE_U8: usize = 1;
pub const SIZE_I8: usize = 1;

pub const SIZE_U16: usize = 2;
pub const SIZE_I16: usize = 2;

pub const SIZE_U32: usize = 4;
pub const SIZE_I32: usize = 4;

pub const SIZE_U64: usize = 8;
pub const SIZE_I64: usize = 8;

pub const SIZE_F32: usize = 4;
pub const SIZE_F64: usize = 8;

pub const SIZE_UOFFSET: usize = SIZE_U32;
pub const SIZE_SOFFSET: usize = SIZE_I32;
pub const SIZE_VOFFSET: usize = SIZE_I16;

pub const SIZE_SIZEPREFIX: usize = SIZE_U32;

pub type SOffsetT = i32;
pub type UOffsetT = u32;
pub type VOffsetT = i16;

pub type HeadUOffsetT = UOffsetT;
pub type TailUOffsetT = UOffsetT;

// enum causes compile error on type mismatch, whereas newtype () would not.
pub enum VectorOffset {}
pub enum StringOffset {}
pub enum ByteStringOffset {}
pub enum UnionOffset {}
pub enum TableOffset {}
pub struct UnionMarker;

pub struct SliceOfGeneratedStruct<T: GeneratedStruct>(T);

pub trait GeneratedStruct {}


#[derive(Debug)]
pub struct Offset<T>(UOffsetT, PhantomData<T>);

// TODO(rw): why do we need to reimplement (with a default impl) Copy to
//           avoid ownership errors?
impl<T> Copy for Offset<T> {}
impl<T> Clone for Offset<T> {
    fn clone(&self) -> Offset<T> {
        Offset::new(self.0.clone())
    }
}
impl<T> PartialEq for Offset<T> {
    fn eq(&self, o: &Offset<T>) -> bool {
        self.value() == o.value()
    }
}

impl<T> Deref for Offset<T> {
    type Target = UOffsetT;
    fn deref(&self) -> &UOffsetT {
        &self.0
    }
}
impl<'a, T: 'a> Offset<T> {
    const BOTTOM: UOffsetT = UOffsetT::max_value();

    pub fn new(o: UOffsetT) -> Offset<T> {
        Offset {
            0: o,
            1: PhantomData,
        }
    }
    pub fn as_union_value(&self) -> Offset<UnionMarker> {
        Offset::new(self.0)
    }
    pub fn value(&self) -> UOffsetT {
        self.0
    }
    pub fn bottom() -> Offset<T> {
        Offset {
            0: Offset::<T>::BOTTOM,
            1: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ForwardsUOffset<T>(UOffsetT, PhantomData<T>); // data unused

#[derive(Debug)]
pub struct ForwardsVOffset<T>(VOffsetT, PhantomData<T>); // data unused

#[derive(Debug)]
pub struct BackwardsSOffset<T>(SOffsetT, PhantomData<T>); // data unused

use follow::Follow;

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsVOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_VOFFSET];
        let off = read_scalar::<VOffsetT>(slice) as usize;
        T::follow(buf, loc + off)
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for BackwardsSOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_SOFFSET];
        let off = read_scalar::<SOffsetT>(slice);
        T::follow(buf, (loc as SOffsetT - off) as usize)
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsUOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_UOFFSET];
        let off = read_scalar::<u32>(slice) as usize;
        T::follow(buf, loc + off)
    }
}

pub struct SkipSizePrefix<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> Follow<'a> for SkipSizePrefix<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc + SIZE_SIZEPREFIX)
    }
}

pub struct SkipRootOffset<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> Follow<'a> for SkipRootOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc + SIZE_UOFFSET)
    }
}

pub struct FileIdentifier;
impl<'a> Follow<'a> for FileIdentifier {
    type Inner = &'a [u8];
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        &buf[loc..loc + FILE_IDENTIFIER_LENGTH]
    }
}

pub struct SkipFileIdentifier<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> Follow<'a> for SkipFileIdentifier<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc + FILE_IDENTIFIER_LENGTH)
    }
}

// Follow trait impls for primitive types.
//
// Ideally, these would be implemented as a single impl using trait bounds on
// EndianScalar, but implementing Follow that way causes a conflict with
// other impls:
//error[E0119]: conflicting implementations of trait `Follow<'_>` for type `&_`:
//     |
//     | impl<'a, T: GeneratedStruct> Follow<'a> for &'a T {
//     | ------------------------------------------------- first implementation here
//...
//     | impl<'a, T: EndianScalar> Follow<'a> for T {
//     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
//     |

impl<'a> Follow<'a> for bool {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for u8 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for u16 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for u32 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for u64 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for i8 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for i16 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for i32 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for i64 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for f32 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}
impl<'a> Follow<'a> for f64 {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        read_scalar_at::<Self>(buf, loc)
    }
}


impl<'a, T: GeneratedStruct> Follow<'a> for &'a T {
    type Inner = &'a T;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let sz = size_of::<T>();
        let buf = &buf[loc..loc + sz];
        let ptr = buf.as_ptr() as *const T;
        unsafe { &*ptr }
    }
}

