use std::marker::PhantomData;

pub trait Table {}
pub struct Verifier {}
pub struct TypeTable {}
pub struct FlatBufferBuilder {}
pub type uoffset_t = usize;
pub struct Offset<T> {
    o: isize,
    phantom: PhantomData<T>,
}
pub fn verify_table_start(_: &Verifier) -> ! {
    unimplemented!()
}
pub fn EndianScalar<T>(x: T) -> ! {
    unimplemented!()
}
pub mod flexbuffers {
    pub struct Reference {}
}
