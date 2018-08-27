pub mod namespace_a {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::marker::PhantomData;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
pub mod namespace_b {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::marker::PhantomData;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EnumInNestedNS {
  A = 0,
  B = 1,
  C = 2
}

const ENUM_MIN_ENUM_IN_NESTED_N_S: i8 = 0;
const ENUM_MAX_ENUM_IN_NESTED_N_S: i8 = 2;

impl<'a> flatbuffers::Follow<'a> for EnumInNestedNS {
  type Inner = Self;
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for EnumInNestedNS {
  fn to_little_endian(self) -> Self {
    let n = i8::to_le(self as i8);
    let p = &n as *const i8 as *const EnumInNestedNS;
    unsafe { *p }
  }
  fn from_little_endian(self) -> Self {
    let n = i8::from_le(self as i8);
    let p = &n as *const i8 as *const EnumInNestedNS;
    unsafe { *p }
  }
}

impl flatbuffers::Push for EnumInNestedNS {
    type Output = EnumInNestedNS;
    #[inline(always)]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<EnumInNestedNS>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_ENUM_IN_NESTED_N_S:[EnumInNestedNS; 3] = [
  EnumInNestedNS::A,
  EnumInNestedNS::B,
  EnumInNestedNS::C
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_ENUM_IN_NESTED_N_S:[&'static str; 3] = [
    "A",
    "B",
    "C"
];

pub fn enum_name_enum_in_nested_n_s(e: EnumInNestedNS) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_ENUM_IN_NESTED_N_S[index]
}

// Size STRUCT_BYTE_SIZE, aligned to 4
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructInNestedNS {
  a_: i32,
  b_: i32,
} // pub struct StructInNestedNS
impl flatbuffers::EndianSafeFollow for StructInNestedNS {}
//type SliceOfStructInNestedNS<'a> = &'a [StructInNestedNS];
//impl<'a> flatbuffers::Follow<'a> for SliceOfStructInNestedNS<'a> {
//    type Inner = &'a [StructInNestedNS];
//    #[inline(always)]
//    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
//        flatbuffers::follow_slice_helper::<StructInNestedNS>(buf, loc)
//    }
//}
impl<'a> flatbuffers::Follow<'a> for StructInNestedNS {
  type Inner = &'a StructInNestedNS;
  #[inline(always)]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::impl_follow_struct::<StructInNestedNS>(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a StructInNestedNS {
  type Inner = &'a StructInNestedNS;
  #[inline(always)]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::impl_follow_struct::<StructInNestedNS>(buf, loc)
  }
}

impl StructInNestedNS {
  pub fn new<'a>(_a: i32, _b: i32) -> Self {
    StructInNestedNS {
      a_: _a.to_little_endian(),
      b_: _b.to_little_endian(),

    }
  }
  pub fn a<'a>(&'a self) -> i32 {
    self.a_.from_little_endian()
  }
  pub fn b<'a>(&'a self) -> i32 {
    self.b_.from_little_endian()
  }
}

impl<'b> flatbuffers::Push for StructInNestedNS {
    type Output = StructInNestedNS;
    #[inline(always)]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const StructInNestedNS as *const u8, self.size())
        };
        dst.copy_from_slice(src);
    }
    #[inline(always)]
    fn size(&self) -> usize {
        ::std::mem::size_of::<StructInNestedNS>()
    }
}
impl<'b> flatbuffers::Push for &'b StructInNestedNS {
    type Output = StructInNestedNS;

    #[inline(always)]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const StructInNestedNS as *const u8, self.size())
        };
        dst.copy_from_slice(src);
    }
    #[inline(always)]
    fn size(&self) -> usize {
        ::std::mem::size_of::<StructInNestedNS>()
    }
}

pub enum TableInNestedNSOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TableInNestedNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for TableInNestedNS<'a> {
    type Inner = TableInNestedNS<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> TableInNestedNS<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInNestedNS {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y TableInNestedNSArgs<'y>) -> flatbuffers::Offset<TableInNestedNS<'x>> {
      let mut builder = TableInNestedNSBuilder::new(_fbb);
      builder.add_foo(args.foo);
      builder.finish()
    }

    pub const VT_FOO: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn foo(&'a self) -> i32 {
    self._tab.get::<i32>(TableInNestedNS::VT_FOO, Some(0)).unwrap()
  }
}

pub struct TableInNestedNSArgs<'a> {
    pub foo: i32,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TableInNestedNSArgs<'a> {
    fn default() -> Self {
        TableInNestedNSArgs {
            foo: 0,
            _phantom: PhantomData,
        }
    }
}
pub struct TableInNestedNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableUnfinishedOffset>,
}
impl<'a: 'b, 'b> TableInNestedNSBuilder<'a, 'b> {
  #[inline(always)]
  pub fn add_foo(&mut self, foo: i32) {
    self.fbb_.push_slot::<i32>(TableInNestedNS::VT_FOO, foo, Some(0));
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInNestedNSBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInNestedNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<TableInNestedNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

}  // pub mod NamespaceB
}  // pub mod NamespaceA

