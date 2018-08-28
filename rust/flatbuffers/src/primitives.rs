use std::marker::PhantomData;
use std::ops::Deref;

use endian_scalar::{read_scalar, read_scalar_at};
use follow::Follow;

pub const FLATBUFFERS_MAX_BUFFER_SIZE: usize = (1u64 << 31) as usize;

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
pub enum TableUnfinishedWIPOffset {}
pub enum TableFinishedWIPOffset {}
pub enum VTableWIPOffset {}
pub enum UnionWIPOffset {}

#[derive(Debug)]
pub struct WIPOffset<T>(UOffsetT, PhantomData<T>);

// TODO(rw): why do we need to reimplement (with a default impl) Copy to
//           avoid ownership errors?
impl<T> Copy for WIPOffset<T> {}
impl<T> Clone for WIPOffset<T> {
    #[inline]
    fn clone(&self) -> WIPOffset<T> {
        WIPOffset::new(self.0.clone())
    }
}
impl<T> PartialEq for WIPOffset<T> {
    fn eq(&self, o: &WIPOffset<T>) -> bool {
        self.value() == o.value()
    }
}

impl<T> Deref for WIPOffset<T> {
    type Target = UOffsetT;
    #[inline]
    fn deref(&self) -> &UOffsetT {
        &self.0
    }
}
impl<'a, T: 'a> WIPOffset<T> {
    #[inline]
    pub fn new(o: UOffsetT) -> WIPOffset<T> {
        WIPOffset {
            0: o,
            1: PhantomData,
        }
    }
    #[inline]
    pub fn as_union_value(&self) -> WIPOffset<UnionWIPOffset> {
        WIPOffset::new(self.0)
    }
    #[inline]
    pub fn value(&self) -> UOffsetT {
        self.0
    }
}

#[derive(Debug)]
pub struct ForwardsUOffset<T>(UOffsetT, PhantomData<T>); // data unused
impl<T> ForwardsUOffset<T> {
    #[inline]
    pub fn value(&self) -> UOffsetT {
        self.0
    }
}

#[derive(Debug)]
pub struct ForwardsVOffset<T>(VOffsetT, PhantomData<T>); // data unused
impl<T> ForwardsVOffset<T> {
    #[inline]
    pub fn value(&self) -> VOffsetT {
        self.0
    }
}

#[derive(Debug)]
pub struct BackwardsSOffset<T>(SOffsetT, PhantomData<T>); // data unused
impl<T> BackwardsSOffset<T> {
    #[inline]
    pub fn value(&self) -> SOffsetT {
        self.0
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsVOffset<T> {
    type Inner = T::Inner;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_VOFFSET];
        let off = read_scalar::<VOffsetT>(slice) as usize;
        T::follow(buf, loc + off)
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for BackwardsSOffset<T> {
    type Inner = T::Inner;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_SOFFSET];
        let off = read_scalar::<SOffsetT>(slice);
        T::follow(buf, (loc as SOffsetT - off) as usize)
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsUOffset<T> {
    type Inner = T::Inner;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_UOFFSET];
        let off = read_scalar::<u32>(slice) as usize;
        T::follow(buf, loc + off)
    }
}

pub struct SkipSizePrefix<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> Follow<'a> for SkipSizePrefix<T> {
    type Inner = T::Inner;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc + SIZE_SIZEPREFIX)
    }
}

pub struct SkipRootOffset<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> Follow<'a> for SkipRootOffset<T> {
    type Inner = T::Inner;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc + SIZE_UOFFSET)
    }
}

pub struct FileIdentifier;
impl<'a> Follow<'a> for FileIdentifier {
    type Inner = &'a [u8];
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        &buf[loc..loc + FILE_IDENTIFIER_LENGTH]
    }
}

pub struct SkipFileIdentifier<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> Follow<'a> for SkipFileIdentifier<T> {
    type Inner = T::Inner;
    #[inline]
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

macro_rules! impl_follow_for_endian_scalar {
    ($ty:ident) => (
        impl<'a> Follow<'a> for $ty {
            type Inner = $ty;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                read_scalar_at::<$ty>(buf, loc)
            }
        }
    )
}

impl_follow_for_endian_scalar!(bool);
impl_follow_for_endian_scalar!(u8);
impl_follow_for_endian_scalar!(u16);
impl_follow_for_endian_scalar!(u32);
impl_follow_for_endian_scalar!(u64);
impl_follow_for_endian_scalar!(i8);
impl_follow_for_endian_scalar!(i16);
impl_follow_for_endian_scalar!(i32);
impl_follow_for_endian_scalar!(i64);
impl_follow_for_endian_scalar!(f32);
impl_follow_for_endian_scalar!(f64);
