extern crate num_traits;

//pub trait ToLittleEndian {
//    fn to_le(self) -> Self {
//        self.to_le()
//    }
//}

//impl ToLittleEndian for i16 {}

use std::marker::PhantomData;

pub trait Table {}
pub struct Verifier {}
impl Verifier {
    pub fn verify<T>(&mut self, _: T) -> bool {
        false
    }
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
    pub fn add_element<T>(&mut self, _: isize, _: T, _: isize) -> usize {
        0
    }
    pub fn add_offset<T>(&mut self, _: isize, _: Offset<T>) -> usize {
        0
    }
    pub fn create_string<T>(&mut self, _: &str) -> Offset<T> {
        Offset::new(0)
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
pub struct Offset<T> (usize, PhantomData<T>);

impl<T> Offset<T> {
    pub fn new(o: usize) -> Self {
        Offset(o, PhantomData)
    }
}

pub fn verify_table_start(_: &Verifier) -> bool {
    false
}
pub fn endian_scalar<T: num_traits::int::PrimInt>(x: T) -> T {
    x.to_le()
}
pub fn write_scalar<S, T>(_: S, _: T) -> ! {
    unimplemented!()
}
pub fn set_field<T>(_: isize, _: T, _: isize) -> ! {
    unimplemented!()
}
pub fn verify_field<T>(_: &Verifier, _: isize) -> bool {
    false
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
pub fn get_field<T>(_: isize, _: isize) -> T {
    unimplemented!()
}
pub fn get_pointer<'a, T: 'a>(_: isize) -> &'a T {
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
