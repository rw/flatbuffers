use std::marker::PhantomData;

pub mod table;
pub use table::{GeneratedStruct, Table};

pub mod endian_scalar;
pub use endian_scalar::{EndianScalar, read_scalar_at, emplace_scalar};

pub mod builder;
pub use builder::*;

pub mod primitives;
pub use primitives::*;

pub mod vector;
pub use vector::*;


#[derive(Debug)]
pub struct ForwardsUOffset<T>(UOffsetT, PhantomData<T>); // data unused

#[derive(Debug)]
pub struct ForwardsVOffset<T>(VOffsetT, PhantomData<T>); // data unused

#[derive(Debug)]
pub struct BackwardsSOffset<T>(SOffsetT, PhantomData<T>); // data unused

impl<'a> Follow<'a> for Table<'a> {
    type Inner = Table<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Table { buf: buf, loc: loc }
    }
}

impl<'a> Follow<'a> for VTable<'a> {
    type Inner = VTable<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        VTable { buf: buf, loc: loc }
    }
}

/// VTableWriter compartmentalizes actions needed to create a vtable.
#[derive(Debug)]
pub struct VTableWriter<'a> {
    buf: &'a mut [u8],
}

impl<'a> VTableWriter<'a> {
    pub fn init(buf: &'a mut [u8]) -> Self {
        VTableWriter { buf: buf }
    }

    /// Writes the vtable length (in bytes) into the vtable.
    ///
    /// Note that callers already need to have computed this to initialize
    /// a VTableWriter.
    ///
    /// In debug mode, asserts that the length of the underlying data is equal
    /// to the provided value.
    #[inline]
    pub fn write_vtable_byte_length(&mut self, n: VOffsetT) {
        emplace_scalar::<VOffsetT>(&mut self.buf[..SIZE_VOFFSET], n);
        debug_assert_eq!(n as usize, self.buf.len());
    }

    /// Writes an object length (in bytes) into the vtable.
    #[inline]
    pub fn write_object_inline_size(&mut self, n: VOffsetT) {
        emplace_scalar::<VOffsetT>(&mut self.buf[SIZE_VOFFSET..2 * SIZE_VOFFSET], n);
    }

    /// Gets an object field offset from the vtable. Only used for debugging.
    ///
    /// Note that this expects field offsets (which are like pointers), not
    /// field ids (which are like array indices).
    #[inline]
    pub fn get_field_offset(&self, vtable_offset: VOffsetT) -> VOffsetT {
        let idx = vtable_offset as usize;
        read_scalar::<VOffsetT>(&self.buf[idx..idx + SIZE_VOFFSET])
    }

    /// Writes an object field offset into the vtable.
    ///
    /// Note that this expects field offsets (which are like pointers), not
    /// field ids (which are like array indices).
    #[inline]
    pub fn write_field_offset(&mut self, vtable_offset: VOffsetT, object_data_offset: VOffsetT) {
        let idx = vtable_offset as usize;
        emplace_scalar::<VOffsetT>(&mut self.buf[idx..idx + SIZE_VOFFSET], object_data_offset);
    }

    /// Clears all data in this VTableWriter. Used to cleanly undo a
    /// vtable write.
    #[inline]
    pub fn clear(&mut self) {
        // This is the closest thing to memset in Rust right now.
        let len = self.buf.len();
        let p = self.buf.as_mut_ptr() as *mut u8;
        unsafe {
            std::ptr::write_bytes(p, 0, len);
        }
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

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsVOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_VOFFSET];
        let off = read_scalar::<u16>(slice) as usize;
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

impl<'a> Follow<'a> for &'a str {
    type Inner = &'a str;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let slice = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len];
        let s = unsafe { std::str::from_utf8_unchecked(slice) };
        s
    }
}

pub struct SliceOfGeneratedStruct<T: GeneratedStruct>(T);

/// Implement direct slice access to structs (they are endian-safe because we
/// use accessors to get their elements).
impl<'a, T: GeneratedStruct + 'a> Follow<'a> for SliceOfGeneratedStruct<T> {
    type Inner = &'a [T];
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let sz = std::mem::size_of::<T>();
        assert!(sz > 0);
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { std::slice::from_raw_parts(ptr, len) };
        s
    }
}

fn follow_slice_helper<T>(buf: &[u8], loc: usize) -> &[T] {
    let sz = std::mem::size_of::<T>();
    debug_assert!(sz > 0);
    let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
    let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
    let ptr = data_buf.as_ptr() as *const T;
    let s: &[T] = unsafe { std::slice::from_raw_parts(ptr, len) };
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

impl<'a, T: GeneratedStruct> Follow<'a> for &'a T {
    type Inner = &'a T;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let sz = std::mem::size_of::<T>();
        let buf = &buf[loc..loc + sz];
        let ptr = buf.as_ptr() as *const T;
        unsafe { &*ptr }
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

pub fn get_root<'a, T: Follow<'a> + 'a>(data: &'a [u8]) -> T::Inner {
    <ForwardsUOffset<T>>::follow(data, 0)
}
pub fn get_size_prefixed_root<'a, T: Follow<'a> + 'a>(data: &'a [u8]) -> T::Inner {
    <SkipSizePrefix<ForwardsUOffset<T>>>::follow(data, 0)
}
pub fn buffer_has_identifier(data: &[u8], ident: &str, size_prefixed: bool) -> bool {
    assert_eq!(ident.len(), FILE_IDENTIFIER_LENGTH);

    let got = if size_prefixed {
        <SkipSizePrefix<SkipRootOffset<FileIdentifier>>>::follow(data, 0)
    } else {
        <SkipRootOffset<FileIdentifier>>::follow(data, 0)
    };

    ident.as_bytes() == got
}

// TODO(rw): figure out better trait bounds for implementing Follow on
//           EndianScalar and GeneratedStruct
// TODO(rw): use macros to impl EndianScalar and as_slice on primitives
