use std::marker::PhantomData;

pub trait Table {}
pub struct Verifier {}
pub struct TypeTable {}
pub struct FlatBufferBuilder {}
pub type uoffset_t = usize;
pub type String = str;
pub type Void<'a> = &'a [u8];
pub struct Vector<T>  {
    phantom: PhantomData<T>,
}
pub struct Offset<T> {
    _o: isize,
    phantom: PhantomData<T>,
}
pub fn verify_table_start(_: &Verifier) -> ! {
    unimplemented!()
}
pub fn endian_scalar<T>(_: T) -> ! {
    unimplemented!()
}
pub fn write_scalar<S, T>(_: S, _: T) -> ! {
    unimplemented!()
}
pub fn set_field<T>(_: isize, _: T, _: isize) -> ! {
    unimplemented!()
}
pub fn verify_field(_: &Verifier, _: isize) -> ! {
    unimplemented!()
}
pub fn verify_offset(_: &Verifier, _: isize) -> ! {
    unimplemented!()
}
pub fn get_struct<T>(_: isize) -> ! {
    unimplemented!()
}
pub fn get_field<T>(_: isize) -> ! {
    unimplemented!()
}
pub fn get_pointer<T>(_: isize) -> ! {
    unimplemented!()
}
pub mod flexbuffers {
    pub struct Reference {}
}
