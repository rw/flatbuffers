pub mod namespace_a {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::marker::PhantomData;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum TableInFirstNSOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TableInFirstNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for TableInFirstNS<'a> {
    type Inner = TableInFirstNS<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> TableInFirstNS<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInFirstNS {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y TableInFirstNSArgs<'y>) -> flatbuffers::Offset<TableInFirstNS<'x>> {
      let mut builder = TableInFirstNSBuilder::new(_fbb);
      if let Some(x) = args.foo_struct { builder.add_foo_struct(x); }
      if let Some(x) = args.foo_table { builder.add_foo_table(x); }
      builder.add_foo_enum(args.foo_enum);
      builder.finish()
    }

    pub const VT_FOO_TABLE: flatbuffers::VOffsetT = 4;
    pub const VT_FOO_ENUM: flatbuffers::VOffsetT = 6;
    pub const VT_FOO_STRUCT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn foo_table(&'a self) -> Option<namespace_b::TableInNestedNS<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<namespace_b::TableInNestedNS<'a>>>(TableInFirstNS::VT_FOO_TABLE, None)
  }
  #[inline]
  pub fn foo_enum(&'a self) -> namespace_b::EnumInNestedNS {
    self._tab.get::<namespace_b::EnumInNestedNS>(TableInFirstNS::VT_FOO_ENUM, Some(namespace_b::EnumInNestedNS::A)).unwrap()
  }
  #[inline]
  pub fn foo_struct(&'a self) -> Option<&'a namespace_b::StructInNestedNS> {
    self._tab.get::<&'a namespace_b::StructInNestedNS>(TableInFirstNS::VT_FOO_STRUCT, None)
  }
}

pub struct TableInFirstNSArgs<'a> {
    pub foo_table: Option<flatbuffers::Offset<namespace_b::TableInNestedNS<'a >>>,
    pub foo_enum: namespace_b::EnumInNestedNS,
    pub foo_struct: Option<&'a  namespace_b::StructInNestedNS>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TableInFirstNSArgs<'a> {
    fn default() -> Self {
        TableInFirstNSArgs {
            foo_table: None,
            foo_enum: namespace_b::EnumInNestedNS::A,
            foo_struct: None,
            _phantom: PhantomData,
        }
    }
}
pub struct TableInFirstNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableUnfinishedOffset>,
}
impl<'a: 'b, 'b> TableInFirstNSBuilder<'a, 'b> {
  #[inline(always)]
  pub fn add_foo_table(&mut self, foo_table: flatbuffers::Offset<namespace_b::TableInNestedNS<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::Offset<namespace_b::TableInNestedNS>>(TableInFirstNS::VT_FOO_TABLE, foo_table);
  }
  #[inline(always)]
  pub fn add_foo_enum(&mut self, foo_enum: namespace_b::EnumInNestedNS) {
    self.fbb_.push_slot::<namespace_b::EnumInNestedNS>(TableInFirstNS::VT_FOO_ENUM, foo_enum, Some(namespace_b::EnumInNestedNS::A));
  }
  #[inline(always)]
  pub fn add_foo_struct(&mut self, foo_struct: &'b  namespace_b::StructInNestedNS) {
    self.fbb_.push_slot_always::<&namespace_b::StructInNestedNS/* foo */>(TableInFirstNS::VT_FOO_STRUCT, foo_struct);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInFirstNSBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInFirstNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<TableInFirstNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

pub enum SecondTableInAOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct SecondTableInA<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for SecondTableInA<'a> {
    type Inner = SecondTableInA<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> SecondTableInA<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        SecondTableInA {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y SecondTableInAArgs<'y>) -> flatbuffers::Offset<SecondTableInA<'x>> {
      let mut builder = SecondTableInABuilder::new(_fbb);
      if let Some(x) = args.refer_to_c { builder.add_refer_to_c(x); }
      builder.finish()
    }

    pub const VT_REFER_TO_C: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn refer_to_c(&'a self) -> Option<super::namespace_c::TableInC<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::namespace_c::TableInC<'a>>>(SecondTableInA::VT_REFER_TO_C, None)
  }
}

pub struct SecondTableInAArgs<'a> {
    pub refer_to_c: Option<flatbuffers::Offset<super::namespace_c::TableInC<'a >>>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for SecondTableInAArgs<'a> {
    fn default() -> Self {
        SecondTableInAArgs {
            refer_to_c: None,
            _phantom: PhantomData,
        }
    }
}
pub struct SecondTableInABuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableUnfinishedOffset>,
}
impl<'a: 'b, 'b> SecondTableInABuilder<'a, 'b> {
  #[inline(always)]
  pub fn add_refer_to_c(&mut self, refer_to_c: flatbuffers::Offset<super::namespace_c::TableInC<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::Offset<super::namespace_c::TableInC>>(SecondTableInA::VT_REFER_TO_C, refer_to_c);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SecondTableInABuilder<'a, 'b> {
    let start = _fbb.start_table();
    SecondTableInABuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<SecondTableInA<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

}  // pub mod NamespaceA

pub mod namespace_c {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::marker::PhantomData;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum TableInCOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TableInC<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for TableInC<'a> {
    type Inner = TableInC<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> TableInC<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInC {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y TableInCArgs<'y>) -> flatbuffers::Offset<TableInC<'x>> {
      let mut builder = TableInCBuilder::new(_fbb);
      if let Some(x) = args.refer_to_a2 { builder.add_refer_to_a2(x); }
      if let Some(x) = args.refer_to_a1 { builder.add_refer_to_a1(x); }
      builder.finish()
    }

    pub const VT_REFER_TO_A1: flatbuffers::VOffsetT = 4;
    pub const VT_REFER_TO_A2: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn refer_to_a1(&'a self) -> Option<super::namespace_a::TableInFirstNS<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::namespace_a::TableInFirstNS<'a>>>(TableInC::VT_REFER_TO_A1, None)
  }
  #[inline]
  pub fn refer_to_a2(&'a self) -> Option<super::namespace_a::SecondTableInA<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::namespace_a::SecondTableInA<'a>>>(TableInC::VT_REFER_TO_A2, None)
  }
}

pub struct TableInCArgs<'a> {
    pub refer_to_a1: Option<flatbuffers::Offset<super::namespace_a::TableInFirstNS<'a >>>,
    pub refer_to_a2: Option<flatbuffers::Offset<super::namespace_a::SecondTableInA<'a >>>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TableInCArgs<'a> {
    fn default() -> Self {
        TableInCArgs {
            refer_to_a1: None,
            refer_to_a2: None,
            _phantom: PhantomData,
        }
    }
}
pub struct TableInCBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableUnfinishedOffset>,
}
impl<'a: 'b, 'b> TableInCBuilder<'a, 'b> {
  #[inline(always)]
  pub fn add_refer_to_a1(&mut self, refer_to_a1: flatbuffers::Offset<super::namespace_a::TableInFirstNS<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::Offset<super::namespace_a::TableInFirstNS>>(TableInC::VT_REFER_TO_A1, refer_to_a1);
  }
  #[inline(always)]
  pub fn add_refer_to_a2(&mut self, refer_to_a2: flatbuffers::Offset<super::namespace_a::SecondTableInA<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::Offset<super::namespace_a::SecondTableInA>>(TableInC::VT_REFER_TO_A2, refer_to_a2);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInCBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInCBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<TableInC<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

}  // pub mod NamespaceC

