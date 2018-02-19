// automatically generated by the FlatBuffers compiler, do not modify



pub mod NamespaceA {
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;
pub mod NamespaceB {
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;

enum EnumInNestedNS {
  A = 0,
  B = 1,
  C = 2
}

const EnumValuesEnumInNestedNS:[EnumInNestedNS; 3] = [
  EnumInNestedNS::A,
  EnumInNestedNS::B,
  EnumInNestedNS::C
];

const EnumNamesEnumInNestedNS:[&'static str; 3] = [
    "A",
    "B",
    "C"
];

fn EnumNameEnumInNestedNS(e: EnumInNestedNS) -> &'static str {
  let index: usize = e as usize;
  EnumNamesEnumInNestedNS[index]
}

// MANUALLY_ALIGNED_STRUCT(4)
#[repr(C, packed)]
pub struct StructInNestedNS {
  a_: i32,
  b_: i32,
} // pub struct StructInNestedNS

impl StructInNestedNS {
  fn Reset(&mut self) {
    //memset(this, 0, size_of(StructInNestedNS));
  }
  fn init(&mut self, _a: i32, _b: i32) {
      self.a_ = flatbuffers::endian_scalar(_a);
      self.b_ = flatbuffers::endian_scalar(_b);

  }
  fn a(&self) -> i32  {
    flatbuffers::endian_scalar(self.a_)
  }
  fn mutate_a(&mut self, _a: i32) {
    flatbuffers::write_scalar(&self.a_, _a);
  }
  fn b(&self) -> i32  {
    flatbuffers::endian_scalar(self.b_)
  }
  fn mutate_b(&mut self, _b: i32) {
    flatbuffers::write_scalar(&self.b_, _b);
  }
}
// STRUCT_END(StructInNestedNS, 8);

pub struct TableInNestedNS {}
impl flatbuffers::Table for TableInNestedNS {}
impl TableInNestedNS /* private flatbuffers::Table */ {
    const VT_FOO: isize = 4;

  fn foo(&self) -> i32  {
    // yo
    flatbuffers::get_field::<i32>(self.VT_FOO, 0)
  }
  fn mutate_foo(&mut self, foo_: i32) -> bool {
    flatbuffers::set_field::<i32>(self.VT_FOO, foo_, 0)
  }
  fn Verify(&self, verifier: &flatbuffers::Verifier) -> bool {
    return flatbuffers::verify_table_start(verifier) &&
           flatbuffers::verify_field::<i32>(verifier, self.VT_FOO) &&
           verifier.end_table();
  }
}

pub struct TableInNestedNSBuilder<'a> {
  fbb_: &'a flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::UOffsetT,
}
impl<'a> TableInNestedNSBuilder<'a> {
  fn add_foo(&mut self, foo: i32 ) {
    self.fbb_.AddElement::<i32>(TableInNestedNS::VT_FOO, foo, 0);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> TableInNestedNSBuilder {
    TableInNestedNSBuilder {
      fbb_: _fbb,
      start_: _fbb.start_table(),
    }
  }
  // TableInNestedNSBuilder &operator=(const TableInNestedNSBuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<TableInNestedNS> {
    let end = self.fbb_.end_table(self.start_);
    let o = flatbuffers::Offset::<TableInNestedNS>::new(end);
    o
  }
}

#[inline]
fn CreateTableInNestedNS(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    foo: i32  /* = 0 */) -> flatbuffers::Offset<TableInNestedNS> {
  let mut builder = TableInNestedNSBuilder::new(_fbb);
  builder.add_foo(foo);
  builder.finish()
}

#[inline]
fn EnumInNestedNSTypeTable() -> /*&mut?*/flatbuffers::TypeTable {
  return flatbuffers::TypeTable{};
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_CHAR, 0, 0 },
    { flatbuffers::ET_CHAR, 0, 0 },
    { flatbuffers::ET_CHAR, 0, 0 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    EnumInNestedNSTypeTable
  };
  static const char *names[] = {
    "A",
    "B",
    "C"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_ENUM, 3, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

#[inline]
fn TableInNestedNSTypeTable() -> /*&mut?*/flatbuffers::TypeTable {
  return flatbuffers::TypeTable{};
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_INT, 0, -1 }
  };
  static const char *names[] = {
    "foo"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 1, type_codes, nullptr, nullptr, names
  };
  return &tt;
  */
}

#[inline]
fn StructInNestedNSTypeTable() -> /*&mut?*/flatbuffers::TypeTable {
  return flatbuffers::TypeTable{};
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_INT, 0, -1 },
    { flatbuffers::ET_INT, 0, -1 }
  };
  static const int32_t values[] = { 0, 4, 8 };
  static const char *names[] = {
    "a",
    "b"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_STRUCT, 2, type_codes, nullptr, values, names
  };
  return &tt;
  */
}

}  // pub mod NamespaceB
}  // pub mod NamespaceA

