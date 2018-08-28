use std::marker::PhantomData;

/// Follow is a trait that allows us to access FlatBuffers in a declarative,
/// type safe, and fast way. They compile down to almost no code (after
/// optimizations). Conceptually, Follow lifts the offset-based access
/// patterns of FlatBuffers data into the type system. This trait is used
/// pervasively at read time, to access tables, vtables, vectors, strings, and
/// all other data. At this time, Follow is not utilized much on the write
/// path.
///
/// Writing a new Follow implementation primarily involves deciding whether
/// you want to return data (of the type Self::Inner) or do you want to
/// continue traversing the FlatBuffer.
pub trait Follow<'a> {
    type Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner;
}

#[inline(always)]
pub fn lifted_follow<'a, T: Follow<'a>>(buf: &'a [u8], loc: usize) -> T::Inner {
    T::follow(buf, loc)
}

#[derive(Debug)]
pub struct FollowStart<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> FollowStart<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self { 0: PhantomData }
    }
    #[inline(always)]
    pub fn self_follow(&'a self, buf: &'a [u8], loc: usize) -> T::Inner {
        T::follow(buf, loc)
    }
}
impl<'a, T: Follow<'a>> Follow<'a> for FollowStart<T> {
    type Inner = T::Inner;
    #[inline(always)]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc)
    }
}
