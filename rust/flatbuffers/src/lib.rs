use std::marker::PhantomData;

pub trait Table {}
pub struct Verifier {}
impl Verifier {
    pub fn end_table(&mut self) -> bool {
        false
    }
}
pub struct TypeTable {}
pub struct FlatBufferBuilder {}
impl FlatBufferBuilder {
    pub fn start_table(&mut self) -> usize {
        0
    }
    pub fn end_table<T>(&mut self, _: T) -> usize {
        0
    }
    pub fn finish<T>(&mut self) -> usize {
        0
    }
}
pub type UOffsetT = usize;
pub type String = i32;
pub type Void<'a> = &'a [u8];
pub struct Vector<T>  {
    phantom: PhantomData<T>,
}
pub struct Offset<T> {
    _o: usize,
    phantom: PhantomData<T>,
}

impl<T> Offset<T> {
    pub fn new(o: usize) -> Self {
        Offset {
            _o: o,
            phantom: PhantomData,
        }
    }
}

pub fn verify_table_start(_: &Verifier) -> bool {
    false
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
pub fn verify_offset_required(_: &Verifier, _: isize) -> ! {
    unimplemented!()
}
pub fn get_root<T>(_: isize) -> ! {
    unimplemented!()
}
pub fn get_mutable_root<T>(_: isize) -> ! {
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
pub fn buffer_has_identifier<S, T>(_: S, _: T) -> ! {
    unimplemented!()
}
pub mod flexbuffers {
    pub struct Reference {}
pub fn get_root<T>(_: isize) -> ! {
    unimplemented!()
}
}
