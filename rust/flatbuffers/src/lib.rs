//extern crate num_traits;
//use std::convert::AsMut;

//pub trait ToLittleEndian {
//    fn to_le(self) -> Self {
//        self.to_le()
//    }
//}

//impl ToLittleEndian for i16 {}


const FLATBUFFERS_MAX_BUFFER_SIZE: usize = ((1u64 << 32) - 1) as usize;

use std::marker::PhantomData;
//use std::iter::FromIterator;

// enum causes compile error on type mismatch, whereas newtype () would not.
pub enum VectorOffset {}
pub enum StringOffset {}
pub enum ByteStringOffset {}
pub enum UnionOffset {}
pub enum TableOffset {}
pub trait GeneratedStruct  : Sized{
    fn to_bytes(&self) -> &[u8] {
        let ptr = &*self as *const Self as *const u8;
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts::<u8>(ptr, std::mem::size_of::<Self>())
        };
        bytes
    }
}
pub trait ElementScalar : Sized + PartialEq + Copy + Clone {
    fn to_le(self) -> Self;
    fn from_le(self) -> Self;
    //fn eq(&self, rhs: &Self) -> bool;
}
//impl ElementScalar for bool { fn to_le(self) -> bool { u8::to_le(self as u8) as bool } }
impl ElementScalar for bool {
    fn to_le(self) -> bool { self }
    fn from_le(self) -> bool { self }
}
impl ElementScalar for u8 {
    fn to_le(self) -> u8 { u8::to_le(self) }
    fn from_le(self) -> u8 { u8::from_le(self) }
}
impl ElementScalar for i8 {
    fn to_le(self) -> i8 { i8::to_le(self) }
    fn from_le(self) -> i8 { i8::from_le(self) }
}
impl ElementScalar for u16 {
    fn to_le(self) -> u16 { u16::to_le(self) }
    fn from_le(self) -> u16 { u16::from_le(self) }
}
impl ElementScalar for i16 {
    fn to_le(self) -> i16 { i16::to_le(self) }
    fn from_le(self) -> i16 { i16::from_le(self) }
}
impl ElementScalar for u32 {
    fn to_le(self) -> u32 { u32::to_le(self) }
    fn from_le(self) -> u32 { u32::from_le(self) }
}
impl ElementScalar for i32 {
    fn to_le(self) -> i32 { i32::to_le(self) }
    fn from_le(self) -> i32 { i32::from_le(self) }
}
impl ElementScalar for u64 {
    fn to_le(self) -> u64 { u64::to_le(self) }
    fn from_le(self) -> u64 { u64::from_le(self) }
}
impl ElementScalar for i64 {
    fn to_le(self) -> i64 { i64::to_le(self) }
    fn from_le(self) -> i64 { i64::from_le(self) }
}
impl ElementScalar for f32 {
    fn to_le(self) -> f32 { f32::to_le(self) }
    fn from_le(self) -> f32 { self } //f32::from_le(self) }
//  fn eq(&self, rhs: &f32) -> bool {
//      let a: u32 = unsafe { std::mem::transmute(*self) };
//      let b: u32 = unsafe { std::mem::transmute(*rhs) };
//      a == b
//  }
}
impl ElementScalar for f64 {
    fn to_le(self) -> f64 { f64::to_le(self) }
    //fn from_le(self) -> f64 { f64::from_le(self) }
    fn from_le(self) -> f64 { self } //f32::from_le(self) }
//  fn eq(&self, rhs: &f64) -> bool {
//      let a: u64 = unsafe { std::mem::transmute(*self) };
//      let b: u64 = unsafe { std::mem::transmute(*rhs) };
//      a == b
//  }
}

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

pub type SOffsetT = i32;
pub type UOffsetT = u32;
pub type VOffsetT = i16;

#[derive(Clone, Copy, Debug)]
struct FieldLoc {
    off: UOffsetT,
    id: VOffsetT,
}

#[inline]
pub fn padding_bytes(buf_size: usize, scalar_size: usize) -> usize {
  // ((!buf_size) + 1) & (scalar_size - 1)
  (!buf_size).wrapping_add(1) & (scalar_size.wrapping_sub(1))
}
pub fn field_index_to_field_offset(field_id: VOffsetT) -> VOffsetT {
    // Should correspond to what end_table() below builds up.
    let fixed_fields = 2;  // Vtable size and Object Size.
    ((field_id + fixed_fields) * (SIZE_VOFFSET as VOffsetT)) as VOffsetT
}
pub fn field_offset_to_field_index(field_o: VOffsetT) -> VOffsetT {
    assert!(field_o >= 2);
    //if field_o == 0 {
    //    return 0;
    //}
    let fixed_fields = 2;  // Vtable size and Object Size.
    (field_o / (SIZE_VOFFSET as VOffsetT)) - fixed_fields
}
pub fn to_bytes<'a, T: 'a + Sized>(t: &'a T) -> &'a [u8] {
    let sz = std::mem::size_of::<T>();
    unsafe {
        std::slice::from_raw_parts((t as *const T) as *const u8, sz)
    }
}
pub fn emplace_scalar<T>(s: &mut [u8], x: T) {
    let sz = std::mem::size_of::<T>();
    let data = unsafe {
        std::slice::from_raw_parts((&x as *const T) as *const u8, sz)
    };

    s[..sz].copy_from_slice(data);
}
pub fn read_scalar_at<T: ElementScalar>(x: &[u8], loc: usize) -> T {
    let buf = &x[loc..loc+std::mem::size_of::<T>()];
    read_scalar(buf)
}
pub fn read_scalar<T: ElementScalar>(x: &[u8]) -> T {
    let p = x.as_ptr();
    let x = unsafe {
        let p2 = std::mem::transmute::<*const u8, *const T>(p);
        (*p2).clone()
    };
    x.from_le()
}

pub struct TypeTable {}
pub struct FlatBufferBuilder<'fbb> {
    pub owned_buf: Vec<u8>,
    pub cur_idx: usize,

    vtable: Vec<UOffsetT>,
    vtables: Vec<UOffsetT>,
    field_locs: Vec<FieldLoc>,

    nested: bool,
    finished: bool,

    min_align: usize,
    //table_end: UOffsetT,
    max_voffset: VOffsetT,
    _phantom: PhantomData<&'fbb ()>,
}
//impl<T> AsMut<T> for FlatBufferBuilder {
//    fn as_mut(&mut self) -> &mut FlatBufferBuilder {
//        self
//    }
//}
impl<'fbb> FlatBufferBuilder<'fbb> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }
    pub fn new_with_capacity(size: usize) -> Self {
        FlatBufferBuilder{
            owned_buf: vec![0u8; size],
            vtable: Vec::new(),
            vtables: Vec::new(),
            field_locs: Vec::new(),

            cur_idx: size,

            nested: false,
            finished: false,

            min_align: 0,
            //table_end: 0,

            max_voffset: 0,
            _phantom: PhantomData,
        }
    }

    fn track_field(&mut self, field_id: VOffsetT, off: UOffsetT) {
        let fl = FieldLoc{id: field_id, off: off};
        self.field_locs.push(fl);
        self.max_voffset = std::cmp::max(self.max_voffset, field_id);
    }
    pub fn start_table(&mut self, num_fields: VOffsetT) -> Offset<TableOffset> {
        self.assert_not_nested();
        self.nested = true;

        self.field_locs.clear();

        self.vtable.clear();
        //self.vtable.truncate(num_fields as usize);
        self.vtable.resize(num_fields as usize, 0);

        Offset::new(self.get_size() as UOffsetT)

        //self.table_end = self.rev_cur_idx();

        //self.get_size() as UOffsetT
    }
    pub fn store_slot(&mut self, slotoff: VOffsetT) {
        unreachable!();
        //let i = slotnum as usize;
        //let i = field_offset_to_field_index(slotnum) as usize;
        let slotnum = field_offset_to_field_index(slotoff) as usize;
        assert!(slotnum < self.vtable.len(), "{} !< {}", self.vtable.len(), slotnum);
        self.vtable[slotnum] = self.rev_cur_idx() as UOffsetT;
    }
    pub fn get_buf_slice(&self) -> &[u8] {
        &self.owned_buf[..]
    }
    pub fn get_active_buf_slice<'a>(&'a self) -> &'a [u8] {
        &self.owned_buf[self.cur_idx..]
    }
    pub fn get_mut_active_buf_slice(&mut self) -> &mut [u8] {
        unreachable!();
        &mut self.owned_buf[self.cur_idx..]
    }
    pub fn reallocate(&mut self, _: usize) {
        unimplemented!()
    }
    pub fn pad(&mut self, n: usize) {
        self.dec_cur_idx(n);
        for i in 0..n {
            self.owned_buf[self.cur_idx + i] = 0;
        }
    }

    pub fn grow_owned_buf(&mut self) {
        let starting_active_size = self.get_size();

        let old_len = self.owned_buf.len();
        let new_len = std::cmp::max(1, old_len * 2);

        assert!(new_len <= FLATBUFFERS_MAX_BUFFER_SIZE,
                "cannot grow buffer beyond 2 gigabytes");
        //assert!(new_len <= 1024,
        //        "cannot grow buffer beyond 1 kilobytes");
        //assert!(new_len <= 1024*1024,
        //        "cannot grow buffer beyond 1 megabytes");

        let diff = new_len - old_len;
        self.owned_buf.resize(new_len, 0);
        //println!("cur_idx += diff: {}, {}", self.cur_idx, diff);
        self.inc_cur_idx(diff);

        let ending_active_size = self.get_size();
        assert_eq!(starting_active_size, ending_active_size);

        if new_len == 1 {
            return;
        }

        // calculate the midpoint, and safely copy the old end data to the new
        // end position:
        let middle = new_len / 2;
        {
            let (left, right) = &mut self.owned_buf[..].split_at_mut(middle);
            //println!("foo {}, {:?}, {:?}", middle, &left[..], &right[..]);
            right.copy_from_slice(left);
        }
        // then, zero out the old end data (just to be safe):
        // should be vectorized by the compiler--rust has no stdlib memset.
        for x in &mut self.owned_buf[..middle] {
            *x = 0;
        }


        //new_len
    }
    //pub fn as_mut(&mut self) -> &mut Self {
    //    self
    //}
    //
    pub fn assert_nested(&self) {
        assert!(self.nested);
        // vtable could be empty (e.g. for all-default values) or in a vector
    }
    pub fn assert_not_nested(&self) {
        assert!(!self.nested);
        assert_eq!(self.field_locs.len(), 0);
    }
    pub fn assert_finished(&self) {
        assert!(self.finished);
    }
    pub fn start_vector(&mut self, len: usize, elem_size: usize) -> UOffsetT {
        self.assert_not_nested();
        self.nested = true;
        //self.prep(SIZE_UOFFSET, elemsize*len);
        //self.prep(alignment, elemsize*len); // Just in case elemsize is wider than uoffset_t.
        self.pre_align(len * elem_size, SIZE_UOFFSET);
        self.pre_align(len * elem_size, elem_size); // Just in case elemsize > uoffset_t.
        self.rev_cur_idx()
    }
    // Offset relative to the end of the buffer.
    pub fn rev_cur_idx(&self) -> UOffsetT {
        (self.owned_buf.len() - self.cur_idx) as UOffsetT
    }
    //pub fn end_vector<'a, 'b, T: Follow<'fbb> + 'fbb>(&'a mut self, num_elems: usize) -> Offset<Vector<'fbb, T>> {
    pub fn end_vector<'a, 'b, T: 'fbb>(&'a mut self, num_elems: usize) -> Offset<Vector<'fbb, T>> {
      self.assert_nested();
      self.nested = false;
      let off = self.push_element_scalar::<UOffsetT>(num_elems as UOffsetT);
      Offset::new(off)


      //   //self.push_element_scalar(num_elems as UOffsetT)


      //   // we already made space for this, so write without PrependUint32
      //   self.push_element_scalar_no_prep(num_elems as UOffsetT);
      //   //self.nested = false;
      //Offset::new(self.rev_cur_idx())
  }
    pub fn emplace_scalar_in_active_buf<T>(&mut self, at: usize, x: T) {
        let buf = &mut self.get_mut_active_buf_slice();
        emplace_scalar(&mut buf[at..], x)
    }
    pub fn pre_align(&mut self, len: usize, alignment: usize) {
        self.track_min_align(alignment);
        let s = self.get_size() as usize;
        self.fill(padding_bytes(s + len, alignment));
    }
  //fn push_small<T: ElementScalar>(&mut self, little_endian_t: T) {
  //  self.make_space(std::mem::size_of::<T>());
  //  emplace_scalar::<T>(kj
  //  *reinterpret_cast<T *>(cur_) = little_endian_t;
  //}
    pub fn prep(&mut self, sz: usize, additional_bytes: usize) {
        unreachable!();
        // Track the biggest thing we've ever aligned to.
        self.min_align = std::cmp::max(self.min_align, sz);

        // Find the amount of alignment needed such that `size` is properly
        // aligned after `additionalBytes`:
        //println!("prep: sz: {}, addl: {}, owned_buf: {}, cur_idx: {}", sz, additional_bytes, self.owned_buf.len(), self.cur_idx);
        let mut align_size = !(self.owned_buf.len() - self.cur_idx + additional_bytes);
        //println!("prep2: align_size == {}", align_size);
        align_size = {
            let (x, _) = align_size.overflowing_add(1);
            x
        };
        align_size &= (sz - 1);
        //println!("align_size: {}", align_size);

        // Reallocate the buffer if needed:
        while self.cur_idx <= align_size+sz+additional_bytes {
            let old_buf_size = self.owned_buf.len();
            self.grow_owned_buf();
            let s = self.owned_buf.len();
            self.inc_cur_idx(s - old_buf_size);
        }
        // pad:
        for i in 0..align_size {
            self.dec_cur_idx(1);
            self.owned_buf[self.cur_idx] = 0;
        }
        //println!("final prep: {}, {}, {}", self.owned_buf.len(), self.cur_idx, align_size);
    }
    #[inline]
    pub fn inc_cur_idx(&mut self, diff: usize) {
        assert!(self.cur_idx <= self.owned_buf.len(), "{}, {}", self.cur_idx, self.owned_buf.len());
        self.cur_idx += diff;
        assert!(self.cur_idx <= self.owned_buf.len(), "{}, {}", self.cur_idx, self.owned_buf.len());
    }
    #[inline]
    pub fn dec_cur_idx(&mut self, diff: usize) {
        assert!(self.cur_idx <= self.owned_buf.len(), "{}, {}", self.cur_idx, self.owned_buf.len());
        self.cur_idx -= diff;
        assert!(self.cur_idx <= self.owned_buf.len(), "{}, {}", self.cur_idx, self.owned_buf.len());
    }
    pub fn get_size(&self) -> usize {
        //println!("{} - {}", self.owned_buf.len(), self.cur_idx);
        let a = self.cur_idx;
        let b = self.owned_buf.len();
        assert!(self.cur_idx <= self.owned_buf.len(), "{}, {}", a, b);
        self.owned_buf.len() - self.cur_idx as usize
        //self.owned_buf.len() - self.cur_idx
    }
    pub fn fill_big(&mut self, zero_pad_bytes: usize) {
        self.fill(zero_pad_bytes);
    }
    pub fn fill(&mut self, zero_pad_bytes: usize) {
        //println!("fill({})", zero_pad_bytes);
        self.make_space(zero_pad_bytes);
        //let start = self.cur_idx;
        //for i in 0..zero_pad_bytes {
        //    self.owned_buf[start + i] = 0;
        //}
    }
    pub fn track_min_align(&mut self, alignment: usize) {
        self.min_align = std::cmp::max(self.min_align, alignment);
    }
    pub fn add_element<T>(&mut self, _: isize, _: T, _: T) -> T {
        unimplemented!()
    }
    pub fn add_offset<T>(&mut self, _: isize, _: Offset<T>) -> usize {

        unimplemented!()
    }
    pub fn add_struct<T>(&mut self, _: VOffsetT, _: T) {
        unreachable!();
        // TODO: unimplemented!()
    }
    // utf-8 string creation
    pub fn create_string<'a, 'b, 'c>(&'a mut self, s: &'b str) -> Offset<&'fbb str> {
        Offset::<&str>::new(self.create_byte_string::<'a, 'b>(s.as_bytes()).value())
    }
    pub fn create_byte_string<'a, 'b, 'c>(&'a mut self, data: &'b [u8]) -> Offset<&'fbb [u8]> {
        self.assert_not_nested();
        self.pre_align(data.len() + 1, SIZE_UOFFSET);  // Always 0-terminated.
        self.fill(1);
        self.push_bytes(data);
        self.push_element_scalar::<UOffsetT>(data.len() as UOffsetT);
        Offset::new(self.get_size() as UOffsetT)
    //return Offset<String>(GetSize());
        //self.assert_not_nested();
        //self.nested = true;
        //let l = data.len();
        //let l_terminated = data.len() + SIZE_U8;
        //self.prep(SIZE_UOFFSET, l_terminated);

        //self.cur_idx -= l_terminated;
        //self.owned_buf[self.cur_idx..self.cur_idx+l].copy_from_slice(data);

        //Offset::new(self.end_vector::<'a, 'b, u8>(l).value())

        ////self.assert_not_nested();
        ////self.nested = true;
        ////self.prep(SIZE_UOFFSET, data.len() + 1);

        ////self.push_element_scalar_no_prep(0u8);
        ////self.push_bytes_no_prep(data);
        ////self.cur_idx -= SIZE_U8;
        ////self.owned_buf[self.cur_idx] = 0;
        ////self.cur_idx -= data.len();
        ////self.owned_buf[self.cur_idx..self.cur_idx+data.len()].copy_from_slice(data);

        ////Offset::new(self.end_vector::<'a, 'b, u8>(data.len()).value())

        ////self.pre_align(data.len() + 1, SIZE_UOFFSET);  // Always 0-terminated.
        //self.push_bytes(data);
        //self.push_element_scalar(data.len() as UOffsetT);
        //LabeledUOffsetT::new(self.get_size() as u32)
    }
    pub fn create_byte_vector<'a, 'b>(&'a mut self, data: &[u8]) -> Offset<Vector<'b, u8>> {
        self.assert_not_nested();
        //self.nested = true;
        self.pre_align(data.len(), SIZE_UOFFSET);
        //self.fill(1);
        self.push_bytes(data);
        self.push_element_scalar::<UOffsetT>(data.len() as UOffsetT);
        Offset::new(self.get_size() as UOffsetT)

        //self.prep(SIZE_UOFFSET, l);

        //self.cur_idx -= l;
        //self.owned_buf[self.cur_idx..self.cur_idx+l].copy_from_slice(data);

        //Offset::new(self.end_vector::<'_, '_, u8>(data.len()).value())
    }
    pub fn create_shared_string<'a>(&mut self, _: &'a str) -> Offset<StringOffset> {
        Offset::new(0)
    }
    //pub fn create_vector_of_strings<'a, 'b, T: 'b>(&'a mut self, _: &'b [T]) -> Offset<&'b [T]> {
    //pub fn create_vector_of_strings<'a>(&mut self, _: &'a [&'a str]) -> LabeledUOffsetT<VectorOffset<StringOffset>> {
    pub fn create_vector_of_strings<'a, 'b, 'c>(&'a mut self, xs: &'b [&'b str]) -> Offset<Vector<'fbb, Offset<&'fbb str>>> {
	unimplemented!()
        // TODO: any way to avoid heap allocs?
        ////let offsets: Vec<Offset<&str>> = xs.iter().map(|s| self.create_string(s)).collect();
        ////self.create_vector(&offsets[..])
        //let offsets: Vec<Offset<FBString>> = vec![];// xs.iter().map(|s| self.create_string(s)).collect();
        //let offsets: Vec<Offset<FBString>> = vec![Offset::new(0); xs.len()];//xs.iter().map(|s| self.create_string(s)).collect();
        //self.create_vector::<'a, 'b, Offset<FBString>>(offsets)
    }
    //pub fn create_vector<T, V: FromIterator<T>>(&mut self, _: V) -> Offset<Vector<T>> {
    // by construction, all items used with this function will already be in little endian format.
    // TODO(rw): trait bounds. maybe require an impl for 'to_le' on everything.
    //pub fn create_vector<'a, T: 'a>(&'a mut self, items: &'a [T]) -> LabeledUOffsetT<&'fbb [T]> {
    //
    //pub fn create_vector<'a, 'b, 'c, T: Follow<'fbb> + 'fbb>(&'a mut self, items: &'b [T]) -> Offset<Vector<'fbb, T>> {
    pub fn create_vector<'a, 'b, 'c, T: 'fbb>(&'a mut self, items: &'b [T]) -> Offset<Vector<'fbb, T>> {
        let elemsize = std::mem::size_of::<T>();
        let start_off = self.start_vector(elemsize, items.len());
        for i in items.iter().rev() {
            self.push_bytes(to_bytes(i));
        }
        Offset::new(self.end_vector::<'_, '_, T>(items.len()).value())
    }
//  //pub fn create_vector_from_fn<'a: 'fbb, 'b, T: 'b, F: FnMut(usize, &mut Self) -> T>(&'fbb mut self, _len: usize, _f: F) -> Offset<&'b [T]> {
    //pub fn create_vector_from_fn<F, T>(&mut self, _len: usize, _f: F) -> Offset<&'fbb [T]>
    //    where F: FnMut(usize, &mut Self) -> T {
    //    Offset::new(0)
    //}
    pub fn create_vector_of_structs<'a, T: 'a>(&'fbb mut self, _: &'a [T]) -> Offset<&'a [T]> {
        unimplemented!();
        //LabeledUOffsetT::new(0)
    }
//  // TODO probably should not be returning [&T]
    pub fn create_vector_of_sorted_structs<'a, T: Follow<'a> + 'a>(&mut self, _: &'a mut [T]) -> Offset<Vector<'a, T>> {
        Offset::new(0)
    }
    pub fn create_vector_of_structs_from_fn<'a, T: Follow<'a> + 'a, F>(&mut self, _len: usize, _f: F) -> Offset<Vector<'a, T>>
        where F: FnMut(usize, &mut T) {
      unimplemented!();
        //Offset::new(0)
    }
    pub fn create_vector_of_sorted_tables<'a, T: Follow<'a> + 'a>(&mut self, _: &'a mut [T]) -> Offset<Vector<'a, T>> {
        unimplemented!();
        //Offset::new(0)
    }
    pub fn dump_buf(&self, label: &str) {
        //println!("dump_buf {}: {}/{}: {:?}", label, self.get_size(), self.owned_buf.len(), self.get_active_buf_slice());
    }
    //pub fn end_table3(&mut self, start: UOffsetT) -> UOffsetT {
    //    self.assert_nested();
    //    self.push_element_scalar::<SOffsetT>(0);
    //    let object_offset = b.get_size();
    //}
    pub fn end_table(&mut self, off: Offset<TableOffset>) -> Offset<TableOffset> {
        //println!("1/3");
        self.assert_nested();
        //println!("2/3");
        let n = self.write_vtable(off.value());
        //println!("3/3");
        self.nested = false;
        self.field_locs.clear();
        let o = Offset::new(n);
        o
    }
    pub fn write_vtable(&mut self, start: UOffsetT) -> UOffsetT {
        // If you get this assert, a corresponding StartTable wasn't called.
        self.assert_nested();
        // Write the vtable offset, which is the start of any Table.
        // We fill it's value later.
        let vtableoffsetloc: UOffsetT = self.push_element_scalar::<SOffsetT>(0xFF);
       // println!("vtableoffsetloc: {}", vtableoffsetloc);
       // println!("field_locs: {:?}", self.field_locs);
        // Write a vtable, which consists entirely of voffset_t elements.
        // It starts with the number of offsets, followed by a type id, followed
        // by the offsets themselves. In reverse:
        // Include space for the last offset and ensure empty tables have a
        // minimum size.
        self.max_voffset = std::cmp::max(self.max_voffset + SIZE_VOFFSET as VOffsetT,
                                         field_index_to_field_offset(0));
        { let s = self.max_voffset; self.fill_big(s as usize); }
        let table_object_size = vtableoffsetloc - start;
        // TODO: always true?
        assert!(table_object_size < 0x10000);  // Vtable use 16bit offsets.
        //WriteScalar<voffset_t>(buf_.data() + sizeof(voffset_t),
        //                       static_cast<voffset_t>(table_object_size));
        emplace_scalar::<VOffsetT>(&mut self.owned_buf[self.cur_idx + SIZE_VOFFSET..],
                                   table_object_size as VOffsetT);

        //   WriteScalar<voffset_t>(buf_.data(), max_voffset_);
        emplace_scalar::<VOffsetT>(&mut self.owned_buf[self.cur_idx..],
                                   self.max_voffset);
        // Write the offsets into the table
        for (i, &fl) in self.field_locs.iter().enumerate() {
            let pos: VOffsetT = (vtableoffsetloc - fl.off) as VOffsetT;
            emplace_scalar::<VOffsetT>(&mut self.owned_buf[self.cur_idx + fl.id as usize..], pos);
        //|  // If this asserts, it means you've set a field twice.
        //|  assert(!ReadScalar<voffset_t>(buf_.data() + field_location->id));
        //|  WriteScalar<voffset_t>(buf_.data() + field_location->id, pos);
        }
        //|ClearOffsets();
        //let vt1 = reinterpret_cast<voffset_t *>(buf_.data());
        //let vt1_size = read_scalar_at::<VOffsetT>(self.get_active_buf_slice());
        let vt_use = self.get_size();
        //   // See if we already have generated a vtable with this exact same
        //   // layout before. If so, make it point to the old one, remove this one.
        //   if (dedup_vtables_) {
        //     for (auto it = buf_.scratch_data(); it < buf_.scratch_end();
        //          it += sizeof(uoffset_t)) {
        //       auto vt_offset_ptr = reinterpret_cast<uoffset_t *>(it);
        //       auto vt2 = reinterpret_cast<voffset_t *>(buf_.data_at(*vt_offset_ptr));
        //       auto vt2_size = *vt2;
        //       if (vt1_size != vt2_size || memcmp(vt2, vt1, vt1_size)) continue;
        //       vt_use = *vt_offset_ptr;
        //       buf_.pop(GetSize() - vtableoffsetloc);
        //       break;
        //     }
        //   }
        //   // If this is a new vtable, remember it.
        //   if (vt_use == GetSize()) { buf_.scratch_push_small(vt_use); }
        // Fill the vtable offset we created above.
        // The offset points from the beginning of the object to where the
        // vtable is stored.
        // Offsets default direction is downward in memory for future format
        // flexibility (storing all vtables at the start of the file).
        //WriteScalar(buf_.data_at(vtableoffsetloc),
        //            static_cast<soffset_t>(vt_use) -
        //                static_cast<soffset_t>(vtableoffsetloc));
        //let idx = self.rev_cur_idx() as usize - vtableoffsetloc as usize;
        //let idx = self.cur_idx as usize + vtableoffsetloc as usize;
        let idx = self.owned_buf.len() - vtableoffsetloc as usize;
        emplace_scalar::<SOffsetT>(&mut self.owned_buf[idx..],
                                   vt_use as SOffsetT - vtableoffsetloc as SOffsetT);

        vtableoffsetloc
    }
    pub fn write_vtable_old(&mut self, table_end: UOffsetT) -> UOffsetT {
        unreachable!();
        self.push_soffset_relative(0);

        let table_offset = self.rev_cur_idx();

        // Trim vtable of trailing zeroes.
        for i in (0..self.vtable.len()).rev() {
            if self.vtable[i] != 0 {
                break
            };
            self.vtable.truncate(i);
        }

        let existing_vtable = false;
        if !existing_vtable {
            // Did not find a vtable, so write this one to the buffer.

            // Write out the current vtable in reverse, because
            // serialization occurs in last-first order:
            // (we cannot use an iterator here due to false borrowing.)
            for i in (0..self.vtable.len()).rev() {
                let val = self.vtable[i]; // prevent underflow of unsigned type
                let vt_off = if val == 0 {
                    0
                } else {
                    // Forward reference to field;
                    // use 32bit number to assert no overflow:
                    table_offset - val
                };
                //println!("pushing VOffsetT {} at index {} (val = {}, table_offset = {})", off, i, val, table_offset);
                self.push_element_scalar::<VOffsetT>(vt_off as VOffsetT);
            }

            // The two metadata fields are written last.

            // First, store the object bytesize:
            let table_size = table_offset - table_end;
            self.push_element_scalar::<VOffsetT>(table_size as VOffsetT);

            // Second, store the vtable bytesize:
            let vtable_size = (self.vtable.len() + VTABLE_METADATA_FIELDS) * SIZE_VOFFSET;
            //let vtable_size = field_index_to_field_offset(self.vtable.len() as VOffsetT);
            self.push_element_scalar::<VOffsetT>(vtable_size as VOffsetT);

            // Next, write the offset to the new vtable in the
            // already-allocated SOffsetT at the beginning of this object:
            let table_start = self.owned_buf.len() as SOffsetT - table_offset as SOffsetT;
            //println!("before emplace: {} {:?}", cur_idx, &self.owned_buf[..]);
            {
                //assert_eq!(0, read_scalar_at::<SOffsetT>(&self.owned_buf[..], table_start as usize));
                let n = self.rev_cur_idx();
                emplace_scalar(&mut self.owned_buf[table_start as usize..],
                               n as SOffsetT - table_offset as SOffsetT);
            }
            //println!("after emplace:  {} {:?}", cur_idx, &self.owned_buf[..]);

            // Finally, store this vtable in memory for future
            // deduplication:
            //{
            //    let n = self.rev_cur_idx();
            //    self.vtables.push(n);
            //}

        //println!("final vtable: {:?}", self.vtable);
            self.vtable.truncate(0);

            return table_offset;
        }

        //// empty vtable for now
        //let table_size = table_offset - off;
        //self.push_element_scalar::<VOffsetT>(table_size as VOffsetT);
        //let vtable_size = (0 + VTABLE_METADATA_FIELDS) * SIZE_VOFFSET;
        //self.push_element_scalar::<VOffsetT>(vtable_size as VOffsetT);
        //let table_start = self.owned_buf.len() as SOffsetT - table_offset as SOffsetT;
        //{
        //    let n = self.rev_cur_idx() as SOffsetT - table_offset as SOffsetT;
        //    emplace_scalar::<SOffsetT>(&mut self.owned_buf[table_start as usize..],n);
        //}

        0
    }
    pub fn end_table_old(&mut self, start: UOffsetT) -> UOffsetT {
        unreachable!();
        // self.assert_nested();

        // let vtableoffsetloc = self.push_element_scalar::<SOffsetT>(0);
        // self.dump_buf(&format!("pushed empty vtableoffsetloc {}", vtableoffsetloc));
        // self.max_voffset = std::cmp::max(self.max_voffset + SIZE_VOFFSET as VOffsetT,
        //                                  field_index_to_field_offset(0));
        // let to_fill = self.max_voffset as usize;
        // self.fill(to_fill);
        // self.dump_buf(&format!("filled {}", to_fill));

        // let table_object_size = vtableoffsetloc - start;
        // assert!(table_object_size < 0x10000);  // Vtable use 16bit offsets.

        // {
        //     let n = table_object_size as VOffsetT;
        //     emplace_scalar::<VOffsetT>(&mut self.get_mut_active_buf_slice()[SIZE_VOFFSET..], n);
        //     self.dump_buf(&format!("after placing table_object_size {}", n));
        // }
        // {
        //     let n = self.max_voffset as VOffsetT;
        //     emplace_scalar::<VOffsetT>(&mut self.get_mut_active_buf_slice(), n);
        //     self.dump_buf(&format!("after placing max_voffset {}", n));
        // }

        // let vt_use = self.get_size();
        // //println!("vt_use at start: {}", vt_use);
        // // TODO write vtable

        // {
        //     let n = (vt_use as SOffsetT) - (vtableoffsetloc as SOffsetT);
        //     //let i = self.get_size() - vtableoffsetloc as usize;
        //     let buf = &mut self.get_mut_active_buf_slice();
        //     //let i = buf.len() - vtableoffsetloc as usize;
        //     let i = vtableoffsetloc as usize;
        //     //println!("writing vt_use... {} at {} -- {:?}", n, i, buf);
        //     emplace_scalar::<SOffsetT>(&mut buf[i..], n);
        // }

        // self.nested = false;

        // vtableoffsetloc as UOffsetT
    }

    pub fn required<T>(&self, _: &Offset<T>, _: VOffsetT) {
        //TODO: unimplemented!()
    }
    pub fn finish<T>(&mut self, root: Offset<T>) {
        self.assert_not_nested();
        self.vtables.clear();
        { let x = self.min_align; self.pre_align(SIZE_UOFFSET, x); }
        //self.push_element_scalar_indirect_uoffset(root.value());
        let fwd = self.refer_to(root.value());
        self.push_element_scalar(fwd);
        self.finished = true;
    }
    pub fn finish_with_identifier<'a, T>(&'a mut self, root: Offset<T>, name: &'static str) {
        self.finish(root)
    }

    pub fn release_buffer_pointer(&mut self) -> DetachedBuffer  {
       //self.Finished();
       // self.buf_.release();
       DetachedBuffer{}
    }

    pub fn push_element_bool(&mut self, b: bool) -> UOffsetT {
        unimplemented!();
        self.push_element_scalar(b as u8)
    }
    fn align(&mut self, elem_size: usize) {
        self.track_min_align(elem_size);
        let s = self.get_size();
        self.fill(padding_bytes(s, elem_size));
    }
    //fn align(&mut self, elem_size: usize) {
    //    let delta = self.cur_idx % elem_size;
    //    self.cur_idx -= delta;
    //}
    pub fn push_element_scalar_no_prep<T: ElementScalar>(&mut self, t: T) -> UOffsetT {
        //let t = t.to_le(); // convert to little-endian
        self.cur_idx -= std::mem::size_of::<T>();
        emplace_scalar::<T>(&mut self.owned_buf[self.cur_idx..], t);
        self.cur_idx as UOffsetT
    }
    pub fn push_element_scalar<T: ElementScalar>(&mut self, t: T) -> UOffsetT {
        //let t = t.to_le();
        self.align(std::mem::size_of::<T>());
        self.push_small(t);
        //self.make_space(sz);
        //emplace_scalar(&mut self.owned_buf[self.cur_idx..], t);
        self.get_size() as UOffsetT
        //self.prep(std::mem::size_of::<T>(), 0);
        //self.cur_idx -= std::mem::size_of::<T>();
        //emplace_scalar::<T>(&mut self.owned_buf[self.cur_idx..], t);
        //self.cur_idx as UOffsetT
    }
    pub fn place_element_scalar<T: ElementScalar>(&mut self, t: T) {
        //let t = t.to_le(); // convert to little-endian
        self.cur_idx -= std::mem::size_of::<T>();
        let cur_idx = self.cur_idx;
        emplace_scalar(&mut self.owned_buf[cur_idx..], t);

    }
    pub fn push_soffset_relative(&mut self, off: SOffsetT) {
        unreachable!();
        self.prep(SIZE_SOFFSET, 0);
        //self.pre_align(SIZE_SOFFSET, 0);
        //self.align(SIZE_SOFFSET);
        //self.align(std::mem::size_of::<SOffsetT>());
        assert!(off <= self.rev_cur_idx() as SOffsetT, "logic error in offsets");
        let off2 = (self.rev_cur_idx() as SOffsetT) - (off as SOffsetT) + (SIZE_SOFFSET as SOffsetT);
        //println!("off2: {}", off2);
        //self.dump_buf("emplace off2");
        self.push_element_scalar_no_prep(off2);
        //emplace_scalar(&mut self.owned_buf[start..start+SIZE_SOFFSET], off2);
    }
    fn push_uoffset_relative(&mut self, off: UOffsetT) {
        unreachable!();
        self.prep(SIZE_UOFFSET, 0);
        assert!(off <= self.rev_cur_idx() as UOffsetT, "logic error in offsets");
        let off2 = (self.rev_cur_idx() as UOffsetT) - (off as UOffsetT) + (SIZE_UOFFSET as UOffsetT);
        //println!("off2: {}", off2);
        //self.dump_buf("emplace off2");
        self.push_element_scalar_no_prep(off2);
        //emplace_scalar(&mut self.owned_buf[start..start+SIZE_SOFFSET], off2);
    }
    fn push_small<T: ElementScalar>(&mut self, x: T) {
        self.make_space(std::mem::size_of::<T>());
        emplace_scalar(&mut self.owned_buf[self.cur_idx..], x);
    }
    // push_bytes_no_prep must not be used when endian-ness is not guaranteed
    // (e.g. with vectors of elements)
    fn push_bytes_no_prep(&mut self, x: &[u8]) -> UOffsetT {
        unreachable!();
        let l = x.len();
        self.cur_idx -= l;
        &mut self.owned_buf[self.cur_idx..self.cur_idx+l].copy_from_slice(x);

        self.cur_idx as UOffsetT
    }
    pub fn push_bytes(&mut self, x: &[u8]) -> UOffsetT {
        let n = self.make_space(x.len());
        &mut self.owned_buf[n..n+x.len()].copy_from_slice(x);

        n as UOffsetT
    }
    pub fn push_slot_scalar_indirect_uoffset(&mut self, slotoff: VOffsetT, x: UOffsetT, default: UOffsetT) {
        if x != default {
            let off = self.push_element_scalar_indirect_uoffset(x);
            self.track_field(slotoff, off);
        }
    }
    pub fn push_element_scalar_indirect_uoffset(&mut self, x: UOffsetT) -> UOffsetT {
        let x = self.refer_to(x);
        return self.push_element_scalar(x);
        self.prep(std::mem::size_of::<UOffsetT>(), 0);
        assert!(x <= self.rev_cur_idx() as UOffsetT, "logic error");
        let off2 = self.rev_cur_idx() as UOffsetT - x + SIZE_UOFFSET as UOffsetT;
        self.push_element_scalar_no_prep::<UOffsetT>(off2)
    }
    pub fn push_slot_bool(&mut self, slotnum: VOffsetT, x: bool, default: bool) {
        unimplemented!();
        self.push_slot_scalar(slotnum, x as u8, default as u8);
    }
    pub fn push_slot_struct<T: Sized>(&mut self, slotoff: VOffsetT, x: &T) {
	// using to_bytes as a trait makes it easier to mix references into T
        self.assert_nested();
        let bytes = to_bytes(x);
        self.align(bytes.len());
       // println!("x bytes: {:?}", x.to_bytes());
        self.push_bytes(bytes);
        let sz = self.get_size() as UOffsetT;
        self.track_field(slotoff, sz);
	//let bytes = x.to_bytes();
        //self.prep(bytes.len(), 0);
        //self.push_bytes_no_prep(bytes);
        //self.store_slot(slotoff);
    }
    // Offsets initially are relative to the end of the buffer (downwards).
    // This function converts them to be relative to the current location
    // in the buffer (when stored here), pointing upwards.
    pub fn refer_to(&mut self, off: UOffsetT) -> UOffsetT {
        // Align to ensure GetSize() below is correct.
        self.align(SIZE_UOFFSET);
        // Offset must refer to something already in buffer.
        assert!(off > 0);
        assert!(off <= self.get_size() as UOffsetT);
        self.get_size() as UOffsetT - off + SIZE_UOFFSET as UOffsetT
    }
    pub fn push_slot_labeled_uoffset_relative_from_option<T>(&mut self, slotoff: VOffsetT, x: Option<Offset<T>>) {
        unimplemented!();
        if let Some(o) = x {
            self.push_slot_labeled_uoffset_relative(slotoff, o)
        }
    }
    pub fn push_slot_offset_relative<T>(&mut self, slotoff: VOffsetT, x: Offset<T>) {
        if x.value() == 0 {
            return;
        }
        let rel_off = self.refer_to(x.value());
        self.push_slot_scalar::<UOffsetT>(slotoff, rel_off, 0);
        //AddElement(field, ReferTo(off.o), static_cast<uoffset_t>(0));
        //self.push_uoffset_relative(x.value());
        //self.track_field(slotoff, off);
        //self.push_slot_scalar::<u32>(slotoff, x.value(), 0)
    }
    pub fn push_slot_labeled_uoffset_relative<T>(&mut self, slotoff: VOffsetT, x: Offset<T>) {
        unreachable!();
        if x.value() == 0 {
            return;
        }
        let rel_off = self.refer_to(x.value());
        self.push_slot_scalar::<UOffsetT>(slotoff, rel_off, 0);
        //AddElement(field, ReferTo(off.o), static_cast<uoffset_t>(0));
        //self.push_uoffset_relative(x.value());
        self.store_slot(slotoff);
        //self.push_slot_scalar::<u32>(slotoff, x.value(), 0)
    }
    pub fn push_slot_scalar<T: ElementScalar + std::fmt::Display>(&mut self, slotoff: VOffsetT, x: T, default: T) {
        //println!("push_slot_scalar: slotnum={}, x={}, default={}, get_active_buf_slice={:?}", slotnum, x, default, self.get_active_buf_slice());
        if x != default {
           //// println!("pushing slot scalar {} != {}", x, default);
            let off = self.push_element_scalar(x);
            //self.prep(std::mem::size_of::<T>(), 0);
            //emplace_scalar(&mut self.owned_buf[self.cur_idx..], x);
            //self.push_element_scalar(x);
            self.track_field(slotoff, off);
        }
    }

    pub fn absolutize_wip_offset<T>(&self, o: Offset<T>) -> UOffsetT {
        unreachable!();
        assert!(self.cur_idx <= self.owned_buf.len());
        let self_front = self.owned_buf.len() as u32 - o.0;
        let diff = self_front - self.cur_idx as u32;
        // and take into account the size of this number...
        (diff + SIZE_UOFFSET as u32) as UOffsetT
    }

    pub fn push<T: Sized>(&mut self, x: T) {
        unreachable!();
        //println!("start of push: {}", self.cur_idx);
        let s = std::mem::size_of::<T>();
        //println!("make space {}", s);
        let n = self.make_space(s);
        {
            let start = self.cur_idx;
            emplace_scalar(&mut self.owned_buf[start..start+s], x);
        }
        //println!("after push: {} {:?} {:?}", self.cur_idx, self.get_active_buf_slice(), &self.owned_buf[..]);
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

    pub fn make_space(&mut self, want: usize) -> usize {
        self.ensure_space(want);
        self.cur_idx -= want;
        self.cur_idx
    }
    pub fn ensure_space(&mut self, want: usize) -> usize {
        assert!(want <= FLATBUFFERS_MAX_BUFFER_SIZE,
		        "cannot grow buffer beyond 2 gigabytes");
        while self.unused_ready_space() < want {
            //println!("growing: {} < {}: {:?}", self.cur_idx, want, self.get_active_buf_slice());
           // println!("growing: {} < {}", self.cur_idx, want);
            self.grow_owned_buf();
           // println!("grew to: {} < {}", self.cur_idx, want);
            //println!("grew to: {}, {}, {:?}", self.cur_idx, self.owned_buf.len(), self.get_active_buf_slice());
        }
        want
    }
    fn unused_ready_space(&self) -> usize {
        assert!(self.owned_buf.len() >= self.get_size());
        self.owned_buf.len() - self.get_size()
    }
    pub fn finished_bytes(&self) -> &[u8] {
        self.assert_finished();
        &self.owned_buf[self.cur_idx..]
    }
}

#[derive(Debug, PartialEq)]
pub struct Offset<T> (UOffsetT, PhantomData<T>);
impl<T> Copy for Offset<T> { } // TODO: why does deriving Copy cause ownership errors?
impl<T> Clone for Offset<T> {
    fn clone(&self) -> Offset<T> {
        Offset::new(self.0.clone())
    }
}

impl<T> std::ops::Deref for Offset<T> {
    type Target = UOffsetT;
    fn deref(&self) -> &UOffsetT {
        &self.0
    }
}
impl<'a, T: 'a> Offset<T> {
    pub fn new(o: UOffsetT) -> Offset<T> {
        Offset { 0: o, 1: PhantomData}
    }
    pub fn union(&self) -> Offset<UnionOffset> {
        Offset::new(self.0)
    }
    pub fn value(&self) -> UOffsetT {
        self.0
    }
}

pub fn endian_scalar<T>(x: T) -> T {
    x
    //x.to_le()
}

#[derive(Debug)]
pub struct FollowStart<T>(PhantomData<T>);
impl<'a, T: Follow<'a> + 'a> FollowStart<T> {
    pub fn new() -> Self {
        Self { 0: PhantomData }
    }
    pub fn self_follow(&'a self, buf: &'a [u8], loc: usize) -> T::Inner {
        T::follow(buf, loc)
    }
}
impl<'a, T: Follow<'a>> Follow<'a> for FollowStart<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        T::follow(buf, loc)
    }
}

#[derive(Debug)]
pub struct ForwardsU32Offset<T>(u32, PhantomData<T>); // data unused

#[derive(Debug)]
pub struct ForwardsU16Offset<T>(u16, PhantomData<T>); // data unused

#[derive(Debug)]
pub struct BackwardsI32Offset<T>(i32, PhantomData<T>); // data unused

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Table<'a> {
    pub buf: &'a [u8],
    pub loc: usize,
}

impl<'a> Follow<'a> for Table<'a> {
    type Inner = Table<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Table { buf: buf, loc: loc }
    }
}

impl<'a> Table<'a> {
    pub fn new(buf: &'a [u8], loc: usize) -> Self {
        Table { buf: buf, loc: loc }
    }
    pub fn vtable(&'a self) -> VTable<'a> {
        <BackwardsI32Offset<VTable<'a>>>::follow(self.buf, self.loc)
    }
    pub fn get<T: Follow<'a> + 'a>(&'a self, slot_byte_loc: VOffsetT, default: Option<T::Inner>) -> Option<T::Inner> {
        assert!(slot_byte_loc as usize >= SIZE_VOFFSET + SIZE_VOFFSET);
        let o = self.vtable().get(slot_byte_loc) as usize;
        if o == 0 {
            return default;
        }
        Some(<T>::follow(self.buf, self.loc + o))
    }
}

#[derive(Debug)]
pub struct VTable<'a> {
    buf: &'a [u8],
    loc: usize,
}

impl<'a> Follow<'a> for VTable<'a> {
    type Inner = VTable<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        //println!("entering follow for VTable with {:?}", &buf[loc..]);
        VTable{buf: buf, loc: loc}
    }
}

impl<'a> VTable<'a> {
    pub fn num_fields(&self) -> usize {
        (self.num_bytes() / SIZE_VOFFSET) - 2
    }
    pub fn num_bytes(&self) -> usize {
        read_scalar_at::<VOffsetT>(self.buf, self.loc) as usize
    }
    pub fn table_inline_num_bytes(&self) -> usize {
        let n = read_scalar_at::<VOffsetT>(self.buf, self.loc + SIZE_VOFFSET);
        n as usize
    }
    pub fn get_field(&self, idx: usize) -> VOffsetT {
        // TODO(rw): distinguish between None and 0?
        if idx > self.num_fields() {
            return 0;
        }
        read_scalar_at::<VOffsetT>(self.buf, self.loc + SIZE_VOFFSET + SIZE_VOFFSET + SIZE_VOFFSET * idx)
    }
    pub fn get(&self, byte_loc: VOffsetT) -> VOffsetT {
        //println!("vtable get byte_loc = {}. num_bytes == {}", byte_loc, self.num_bytes());
        // TODO(rw): distinguish between None and 0?
        if byte_loc as usize >= self.num_bytes() {
            return 0;
        }
        read_scalar_at::<VOffsetT>(self.buf, self.loc + byte_loc as usize)
    }
}

//pub trait Push<'a> {
//    type Outer;
//    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner;
//}

pub trait Follow<'a> {
    type Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner;
}

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsU32Offset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
       //println!("entering follow for ForwardsU32Offset<T> with {:?}", &buf[loc..]);
        let slice = &buf[loc..loc + SIZE_UOFFSET];
        let off = read_scalar::<u32>(slice) as usize;
        T::follow(buf, loc + off)
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsU16Offset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
       //println!("entering follow for ForwardsU16Offset<T> with {:?}", &buf[loc..]);
        let slice = &buf[loc..loc + 2];
        let off = read_scalar::<u16>(slice) as usize;
        T::follow(buf, loc + off)
    }
}
impl<'a, T: Follow<'a>> Follow<'a> for BackwardsI32Offset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        //println!("entering follow for ForwardsI32Offset<T> with {:?}", &buf[loc..]);
        let slice = &buf[loc..loc + 4];
        let off = read_scalar::<i32>(slice);
        T::follow(buf, (loc as i32 - off) as usize)
    }
}
impl<'a> Follow<'a> for &'a str {
    type Inner = &'a str;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        //println!("entering follow for &'a str with {:?}", &buf[loc..]);
        let len = read_scalar::<u32>(&buf[loc..loc + 4]) as usize;
        let slice = &buf[loc + 4..loc + 4 + len];
        let s = unsafe { std::str::from_utf8_unchecked(slice) };
        s
    }
}

impl<'a, T: Sized> Follow<'a> for &'a [T] {
    type Inner = &'a [T];
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        //println!("entering follow for &'a [T] with {:?}", &buf[loc..]);
        let sz = std::mem::size_of::<T>();
        assert!(sz > 0);
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let data_buf = &buf[loc + SIZE_UOFFSET .. loc + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { std::slice::from_raw_parts(ptr, len) };
        s
    }
}

impl<'a, T: Follow<'a> + 'a> Follow<'a> for Vector<'a, T> {
    type Inner = Vector<'a, T>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        //println!("entering follow for Vector<T> with {:?}", &buf[loc..]);
        Vector::new(buf, loc)
    }
}

impl<'a, T: Follow<'a>> Vector<'a, T> {
    pub fn new(buf: &'a [u8], loc: usize) -> Self {
        Vector{0: buf, 1: loc, 2: PhantomData}
    }
    pub fn len(&self) -> usize {
        read_scalar::<u32>(&self.0[self.1 as usize..]) as usize
    }
    pub fn get(&self, idx: usize) -> T::Inner {
        assert!(idx < read_scalar::<u32>(&self.0[self.1 as usize..]) as usize);
        //println!("entering get({}) with {:?}", idx, &self.0[self.1 as usize..]);
        let sz = std::mem::size_of::<T>();
        assert!(sz > 0);
        T::follow(self.0, self.1 as usize + 4 + sz * idx)
    }

    pub fn as_slice_unfollowed(&'a self) -> &'a [T] {
        let sz = std::mem::size_of::<T>();
        assert!(sz > 0);
        let len = self.len();
        let data_buf = &self.0[self.1 + SIZE_UOFFSET .. self.1 + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { std::slice::from_raw_parts(ptr, len) };
        s
    }
    pub fn into_slice_unfollowed(self) -> &'a [T] {
        let sz = std::mem::size_of::<T>();
        assert!(sz > 0);
        let len = self.len();
        let data_buf = &self.0[self.1 + SIZE_UOFFSET .. self.1 + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { std::slice::from_raw_parts(ptr, len) };
        s
    }
}

impl<'a, T: Sized> Follow<'a> for &'a T {
    type Inner = &'a T;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let sz = std::mem::size_of::<T>();
        let buf = &buf[loc..loc + sz];
        //println!("entering follow for Sized ref with {:?}", buf);
        let ptr = buf.as_ptr() as *const T;
        unsafe { &*ptr }
    }
}

// implementing these using bounds causes them to conflict with the Sized impl
impl<'a> Follow<'a> for bool { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for u8   { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for u16  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for u32  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for u64  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for i8   { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for i16  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for i32  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for i64  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for f32  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }
impl<'a> Follow<'a> for f64  { type Inner = Self; fn follow(buf: &'a [u8], loc: usize) -> Self::Inner { read_scalar_at::<Self>(buf, loc) } }

#[derive(Debug)]
pub struct Vector<'a, T: Sized + 'a>(&'a [u8], usize, PhantomData<T>);

pub fn lifted_follow<'a, T: Follow<'a>>(buf: &'a [u8], loc: usize) -> T::Inner {
    T::follow(buf, loc)
}
//pub fn get_field<T: num_traits::Num>(_: isize, _: T) -> T {
//    unimplemented!()
//}
//pub fn get_field_mut<T: num_traits::Num>(_: isize, _: T) -> T {
//    unimplemented!()
//}
pub fn get_pointer<'a, T: 'a>(_: VOffsetT) -> &'a T {
    unimplemented!()
}
pub fn get_pointer_mut<'a, T: 'a>(_: VOffsetT) -> &'a mut T {
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


