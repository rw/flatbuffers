extern crate num_traits;
//use std::convert::AsMut;

//pub trait ToLittleEndian {
//    fn to_le(self) -> Self {
//        self.to_le()
//    }
//}

//impl ToLittleEndian for i16 {}


use std::marker::PhantomData;
//use std::iter::FromIterator;

pub trait Table {}
pub struct Verifier {}
impl Verifier {
    pub fn new() -> Self {
        Verifier{}
    }
    pub fn verify<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn verify_buffer<T>(&mut self, _: &'static str) -> bool {
        false
    }
    pub fn verify_vector_of_strings<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn verify_vector_of_tables<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn verify_table<T>(&mut self, _: T) -> bool {
        false
    }
    pub fn end_table(&mut self) -> bool {
        false
    }
}
pub struct TypeTable {}
pub struct FlatBufferBuilder<'fbb> {
    _phantom: PhantomData<&'fbb ()>,
}
//impl<T> AsMut<T> for FlatBufferBuilder {
//    fn as_mut(&mut self) -> &mut FlatBufferBuilder {
//        self
//    }
//}
impl<'fbb> FlatBufferBuilder<'fbb> {
    pub fn new() -> Self {
        FlatBufferBuilder{
            _phantom: PhantomData,
        }
    }
    pub fn start_table(&mut self) -> UOffsetT {
        0
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    pub fn add_element<T>(&mut self, _: isize, _: T, _: T) -> T {
        unimplemented!()
    }
    pub fn add_offset<T>(&mut self, _: isize, _: Offset<T>) -> usize {
        unimplemented!()
    }
    pub fn add_struct<T>(&mut self, _: isize, _: T) {
        unimplemented!()
    }
    pub fn create_string<'a>(&mut self, _: &'a str) -> Offset<String<'fbb>> {
        Offset::new(0)
    }
    pub fn create_shared_string<'a>(&mut self, _: &'a str) -> Offset<String<'fbb>> {
        Offset::new(0)
    }
    //pub fn create_vector_of_strings<'a, 'b, T: 'b>(&'a mut self, _: &'b [T]) -> Offset<&'b [T]> {
    pub fn create_vector_of_strings<'a>(&mut self, _: &'a [&'a str]) -> Offset<&'fbb [Offset<String<'fbb>>]> {
        Offset::new(0)
    }
    //pub fn create_vector<T, V: FromIterator<T>>(&mut self, _: V) -> Offset<Vector<T>> {
    pub fn create_vector<'a, T: 'a>(&'a mut self, _: &'a [T]) -> Offset<&'fbb [T]> {
        Offset::new(0)
    }
//  //pub fn create_vector_from_fn<'a: 'fbb, 'b, T: 'b, F: FnMut(usize, &mut Self) -> T>(&'fbb mut self, _len: usize, _f: F) -> Offset<&'b [T]> {
    pub fn create_vector_from_fn<F, T>(&mut self, _len: usize, _f: F) -> Offset<&'fbb [T]>
        where F: FnMut(usize, &mut Self) -> T {
        Offset::new(0)
    }
//  pub fn create_vector_of_structs<'a, T: 'a>(&'fbb mut self, _: &'a [T]) -> Offset<&'a [T]> {
//      Offset::new(0)
//  }
//  // TODO probably should not be returning [&T]
    pub fn create_vector_of_sorted_structs<'a, T>(&mut self, _: &'a mut [T]) -> Offset<&'fbb [&'fbb T]> {
        Offset::new(0)
    }
    pub fn create_vector_of_structs_from_fn<T, F>(&mut self, _len: usize, _f: F) -> Offset<&'fbb [&'fbb T]>
        where F: FnMut(usize, &mut T) {
        Offset::new(0)
    }
    pub fn create_vector_of_sorted_tables<'a, T>(&mut self, _: &'a mut [T]) -> Offset<&'fbb [T]> {
        Offset::new(0)
    }
    pub fn end_table<T>(&mut self, _: T) -> usize {
        0
    }
    pub fn required<T>(&self, _: &Offset<T>, _: isize) -> bool {
        unimplemented!()
    }
    pub fn finish<T>(&mut self, _root: Offset<T>)  {
    }
    pub fn finish_with_identifier<'a, T>(&'a mut self, _root: Offset<T>, _name: &'static str) {
    }

    pub fn release_buffer_pointer(&mut self) -> DetachedBuffer  {
       //self.Finished();
       // self.buf_.release();
       DetachedBuffer{}
    }

    pub fn release(&mut self) {
        //DetachedBuffer fb(allocator_, own_allocator_, buf_, reserved_, cur_,
        //                  size());
        //allocator_ = nullptr;
        //own_allocator_ = false;
        //buf_ = nullptr;
        //clear();
        //return fb;
    }
}
pub trait UOffsetTTrait {}
pub trait OffsetTTrait {}
pub trait VOffsetTTrait {}
pub type UOffsetT = u32;
pub type OffsetT = i32;
pub type VOffsetT = i16;

pub type String<'a> = &'a str;
pub type Void<'a> = &'a [u8];
pub struct Vector<T>  {
    phantom: PhantomData<T>,
}

pub struct Offset<T> (usize, PhantomData<T>);
pub struct UOffset<T> (u32, PhantomData<T>);
impl<T> Copy for Offset<T> { }

impl<T> Clone for Offset<T> {
    fn clone(&self) -> Offset<T> {
        *self
    }
}

impl<T> Offset<T> {
    pub fn new(o: usize) -> Self {
        Offset(o, PhantomData)
    }
    pub fn union(&self) -> Offset<Void> {
        Offset::new(self.0)
    }
}
impl<T> UOffset<T> {
    pub fn new(o: u32) -> Self {
        UOffset(o, PhantomData)
    }
}

//impl<T> From<usize> for UOffset<T> { fn from(n: usize) -> Self { UOffset::new(n) } }
//impl<T> From<isize> for Offset<T> { fn from(n: isize) -> Self { Offset::new(n) } }
//impl<T> From<u8> for Offset<T>  { fn from(n: u8)  -> Self { Offset::new(n) } }
//impl<T> From<u16> for Offset<T> { fn from(n: u16) -> Self { Offset::new(n) } }
//impl<T> From<u32> for Offset<T> { fn from(n: u32) -> Self { Offset::new(n) } }
//impl<T> From<u64> for Offset<T> { fn from(n: u64) -> Self { Offset::new(n) } }
//impl<T> From<i8> for Offset<T>  { fn from(n: i8)  -> Self { Offset::new(n) } }
//impl<T> From<i16> for Offset<T> { fn from(n: i16) -> Self { Offset::new(n) } }
//impl<T> From<i32> for Offset<T> { fn from(n: i32) -> Self { Offset::new(n) } }
//impl<T> From<i64> for Offset<T> { fn from(n: i64) -> Self { Offset::new(n) } }
//impl<T> From<usize> for Offset<T> { fn from(n: usize) -> Self { Offset::new(n) } }
//impl<T> From<isize> for Offset<T> { fn from(n: isize) -> Self { Offset::new(n) } }
//impl From<usize> for Offset<u16> { fn from(n: usize) -> Self { Offset::new(n) } }
//impl From<usize> for Offset<u32> { fn from(n: usize) -> Self { Offset::new(n) } }
//impl From<usize> for Offset<u64> { fn from(n: usize) -> Self { Offset::new(n) } }
//impl From<usize> for Offset<f32> { fn from(n: usize) -> Self { Offset::new(n) } }
//impl From<usize> for Offset<f64> { fn from(n: usize) -> Self { Offset::new(n) } }

pub fn verify_table_start(_: &Verifier) -> bool {
    false
}
pub fn endian_scalar<T: num_traits::int::PrimInt>(x: T) -> T {
    x.to_le()
}
pub fn write_scalar<S, T>(_: S, _: T) -> ! {
    unimplemented!()
}
pub fn set_field<T>(_: isize, _: T, _: T) -> bool {
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
pub fn get_root<T>(_: &[u8]) -> T {
    unimplemented!()
}
pub fn get_mutable_root<T>(_: &[u8]) -> T {
    unimplemented!()
}
pub fn get_struct<T>(_: isize) -> T {
    unimplemented!()
}
pub fn get_struct_mut<T>(_: isize) -> T {
    unimplemented!()
}
pub fn get_field<T: num_traits::Num>(_: isize, _: T) -> T {
    unimplemented!()
}
pub fn get_field_mut<T: num_traits::Num>(_: isize, _: T) -> T {
    unimplemented!()
}
pub fn get_pointer<'a, T: 'a>(_: isize) -> &'a T {
    unimplemented!()
}
pub fn get_pointer_mut<'a, T: 'a>(_: isize) -> &'a mut T {
    unimplemented!()
}
pub fn buffer_has_identifier<S, T>(_: S, _: T) -> bool {
    false
}
pub struct DetachedBuffer {}
pub mod flexbuffers {
    pub struct Reference {}
pub fn get_root<T>(_: &[u8], _: isize) -> T {
    unimplemented!()
}

}
