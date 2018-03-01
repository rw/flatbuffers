// automatically generated by the FlatBuffers compiler, do not modify



#include "namespace_test1_generated.rs"

pub mod NamespaceA {
  #[allow(unused_imports)]
  use std::mem;
  #[allow(unused_imports)]
  use std::marker::PhantomData;
  #[allow(unused_imports)]
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;

pub struct TableInFirstNS {}
impl flatbuffers::Table for TableInFirstNS {}
impl TableInFirstNS /* private flatbuffers::Table */ {
    const VT_FOO_TABLE: isize = 4;
    const VT_FOO_ENUM: isize = 6;
    const VT_FOO_STRUCT: isize = 8;

  fn foo_table(&self) -> &NamespaceB::TableInNestedNS  {
    // yo
    flatbuffers::get_pointer::<&NamespaceB::TableInNestedNS>(TableInFirstNS::VT_FOO_TABLE)
  }
  fn mutable_foo_table<'a>(&'a mut self) -> &'a mut NamespaceB::TableInNestedNS  {
    /* TODO: are there non-reference choices here? */
    &mut flatbuffers::get_pointer_mut::<&'a mut NamespaceB::TableInNestedNS >(TableInFirstNS::VT_FOO_TABLE)
  }
  fn foo_enum(&self) -> NamespaceA::NamespaceB::EnumInNestedNS  {
    // yo
    EnumValuesNamespaceA::NamespaceB::EnumInNestedNS[flatbuffers::get_field::<i8>(TableInFirstNS::VT_FOO_ENUM, 0) as usize]
  }
  fn mutate_foo_enum(&mut self, foo_enum_: NamespaceA::NamespaceB::EnumInNestedNS) -> bool {
    flatbuffers::set_field::<i8>(TableInFirstNS::VT_FOO_ENUM, foo_enum_ as i8, 0)
  }
  fn foo_struct(&self) -> &NamespaceB::StructInNestedNS  {
    // yo
    flatbuffers::get_struct::<&NamespaceB::StructInNestedNS>(TableInFirstNS::VT_FOO_STRUCT)
  }
  fn mutable_foo_struct<'a>(&'a mut self) -> &'a mut NamespaceB::StructInNestedNS  {
    /* TODO: are there non-reference choices here? */
    &mut flatbuffers::get_struct_mut::<&'a mut NamespaceB::StructInNestedNS >(TableInFirstNS::VT_FOO_STRUCT)
  }
  fn Verify(&self, verifier: &mut flatbuffers::Verifier) -> bool {
    return flatbuffers::verify_table_start(verifier) &&
           flatbuffers::verify_offset(verifier, TableInFirstNS::VT_FOO_TABLE) &&
           verifier.verify_table(self.foo_table()) &&
           flatbuffers::verify_field::<i8>(verifier, TableInFirstNS::VT_FOO_ENUM) &&
           flatbuffers::verify_field::<NamespaceB::StructInNestedNS>(verifier, TableInFirstNS::VT_FOO_STRUCT) &&
           verifier.end_table();
  }
}

pub struct TableInFirstNSBuilder<'a> {
  fbb_: &'a mut flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::UOffsetT,
}
impl<'a> TableInFirstNSBuilder<'a> {
  fn add_foo_table(&mut self, foo_table: flatbuffers::Offset<NamespaceB::TableInNestedNS> ) {
    self.fbb_.add_offset(TableInFirstNS::VT_FOO_TABLE, foo_table);
  }
  fn add_foo_enum(&mut self, foo_enum: NamespaceA::NamespaceB::EnumInNestedNS ) {
    self.fbb_.add_element::<i8>(TableInFirstNS::VT_FOO_ENUM, foo_enum as i8, 0);
  }
  fn add_foo_struct(&mut self, foo_struct: &NamespaceB::StructInNestedNS) {
    self.fbb_.add_struct(TableInFirstNS::VT_FOO_STRUCT, foo_struct);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> TableInFirstNSBuilder {
    let start = _fbb.start_table();
    TableInFirstNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // TableInFirstNSBuilder &operator=(const TableInFirstNSBuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<TableInFirstNS> {
    let end = self.fbb_.end_table(self.start_);
    let o = flatbuffers::Offset::<TableInFirstNS>::new(end);
    o
  }
}

#[inline]
fn CreateTableInFirstNS(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    foo_table: flatbuffers::Offset<NamespaceB::TableInNestedNS>  /* = 0 */,
    foo_enum: NamespaceA::NamespaceB::EnumInNestedNS  /* = NamespaceA::NamespaceB::EnumInNestedNS::A */,
    foo_struct: &NamespaceB::StructInNestedNS /* = 0 */) -> flatbuffers::Offset<TableInFirstNS> {
  let mut builder = TableInFirstNSBuilder::new(_fbb);
  builder.add_foo_struct(foo_struct);
  builder.add_foo_table(foo_table);
  builder.add_foo_enum(foo_enum);
  builder.finish()
}

pub struct SecondTableInA {}
impl flatbuffers::Table for SecondTableInA {}
impl SecondTableInA /* private flatbuffers::Table */ {
    const VT_REFER_TO_C: isize = 4;

  fn refer_to_c(&self) -> &super::NamespaceC::TableInC  {
    // yo
    flatbuffers::get_pointer::<&super::NamespaceC::TableInC>(SecondTableInA::VT_REFER_TO_C)
  }
  fn mutable_refer_to_c<'a>(&'a mut self) -> &'a mut super::NamespaceC::TableInC  {
    /* TODO: are there non-reference choices here? */
    &mut flatbuffers::get_pointer_mut::<&'a mut super::NamespaceC::TableInC >(SecondTableInA::VT_REFER_TO_C)
  }
  fn Verify(&self, verifier: &mut flatbuffers::Verifier) -> bool {
    return flatbuffers::verify_table_start(verifier) &&
           flatbuffers::verify_offset(verifier, SecondTableInA::VT_REFER_TO_C) &&
           verifier.verify_table(self.refer_to_c()) &&
           verifier.end_table();
  }
}

pub struct SecondTableInABuilder<'a> {
  fbb_: &'a mut flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::UOffsetT,
}
impl<'a> SecondTableInABuilder<'a> {
  fn add_refer_to_c(&mut self, refer_to_c: flatbuffers::Offset<super::NamespaceC::TableInC> ) {
    self.fbb_.add_offset(SecondTableInA::VT_REFER_TO_C, refer_to_c);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> SecondTableInABuilder {
    let start = _fbb.start_table();
    SecondTableInABuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // SecondTableInABuilder &operator=(const SecondTableInABuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<SecondTableInA> {
    let end = self.fbb_.end_table(self.start_);
    let o = flatbuffers::Offset::<SecondTableInA>::new(end);
    o
  }
}

#[inline]
fn CreateSecondTableInA(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    refer_to_c: flatbuffers::Offset<super::NamespaceC::TableInC>  /* = 0 */) -> flatbuffers::Offset<SecondTableInA> {
  let mut builder = SecondTableInABuilder::new(_fbb);
  builder.add_refer_to_c(refer_to_c);
  builder.finish()
}

#[inline]
fn TableInFirstNSTypeTable() -> /*&mut?*/flatbuffers::TypeTable {
  return flatbuffers::TypeTable{};
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_SEQUENCE, 0, 0 },
    { flatbuffers::ET_CHAR, 0, 1 },
    { flatbuffers::ET_SEQUENCE, 0, 2 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    NamespaceA::NamespaceB::TableInNestedNSTypeTable,
    NamespaceA::NamespaceB::EnumInNestedNSTypeTable,
    NamespaceA::NamespaceB::StructInNestedNSTypeTable
  };
  static const char *names[] = {
    "foo_table",
    "foo_enum",
    "foo_struct"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 3, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

#[inline]
fn SecondTableInATypeTable() -> /*&mut?*/flatbuffers::TypeTable {
  return flatbuffers::TypeTable{};
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_SEQUENCE, 0, 0 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    NamespaceC::TableInCTypeTable
  };
  static const char *names[] = {
    "refer_to_c"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 1, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

}  // pub mod NamespaceA

pub mod NamespaceC {
  #[allow(unused_imports)]
  use std::mem;
  #[allow(unused_imports)]
  use std::marker::PhantomData;
  #[allow(unused_imports)]
  #[allow(unreachable_code)]
  extern crate flatbuffers;
  #[allow(unused_imports)]
  use self::flatbuffers::flexbuffers;
  #[allow(unused_imports)]
  use std::cmp::Ordering;

pub struct TableInC {}
impl flatbuffers::Table for TableInC {}
impl TableInC /* private flatbuffers::Table */ {
    const VT_REFER_TO_A1: isize = 4;
    const VT_REFER_TO_A2: isize = 6;

  fn refer_to_a1(&self) -> &super::NamespaceA::TableInFirstNS  {
    // yo
    flatbuffers::get_pointer::<&super::NamespaceA::TableInFirstNS>(TableInC::VT_REFER_TO_A1)
  }
  fn mutable_refer_to_a1<'a>(&'a mut self) -> &'a mut super::NamespaceA::TableInFirstNS  {
    /* TODO: are there non-reference choices here? */
    &mut flatbuffers::get_pointer_mut::<&'a mut super::NamespaceA::TableInFirstNS >(TableInC::VT_REFER_TO_A1)
  }
  fn refer_to_a2(&self) -> &super::NamespaceA::SecondTableInA  {
    // yo
    flatbuffers::get_pointer::<&super::NamespaceA::SecondTableInA>(TableInC::VT_REFER_TO_A2)
  }
  fn mutable_refer_to_a2<'a>(&'a mut self) -> &'a mut super::NamespaceA::SecondTableInA  {
    /* TODO: are there non-reference choices here? */
    &mut flatbuffers::get_pointer_mut::<&'a mut super::NamespaceA::SecondTableInA >(TableInC::VT_REFER_TO_A2)
  }
  fn Verify(&self, verifier: &mut flatbuffers::Verifier) -> bool {
    return flatbuffers::verify_table_start(verifier) &&
           flatbuffers::verify_offset(verifier, TableInC::VT_REFER_TO_A1) &&
           verifier.verify_table(self.refer_to_a1()) &&
           flatbuffers::verify_offset(verifier, TableInC::VT_REFER_TO_A2) &&
           verifier.verify_table(self.refer_to_a2()) &&
           verifier.end_table();
  }
}

pub struct TableInCBuilder<'a> {
  fbb_: &'a mut flatbuffers::FlatBufferBuilder,
  start_: flatbuffers::UOffsetT,
}
impl<'a> TableInCBuilder<'a> {
  fn add_refer_to_a1(&mut self, refer_to_a1: flatbuffers::Offset<super::NamespaceA::TableInFirstNS> ) {
    self.fbb_.add_offset(TableInC::VT_REFER_TO_A1, refer_to_a1);
  }
  fn add_refer_to_a2(&mut self, refer_to_a2: flatbuffers::Offset<super::NamespaceA::SecondTableInA> ) {
    self.fbb_.add_offset(TableInC::VT_REFER_TO_A2, refer_to_a2);
  }
  fn new(_fbb: &mut flatbuffers::FlatBufferBuilder) -> TableInCBuilder {
    let start = _fbb.start_table();
    TableInCBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  // TableInCBuilder &operator=(const TableInCBuilder &);
  fn finish(&mut self) -> flatbuffers::Offset<TableInC> {
    let end = self.fbb_.end_table(self.start_);
    let o = flatbuffers::Offset::<TableInC>::new(end);
    o
  }
}

#[inline]
fn CreateTableInC(
    _fbb: &mut flatbuffers::FlatBufferBuilder,
    refer_to_a1: flatbuffers::Offset<super::NamespaceA::TableInFirstNS>  /* = 0 */,
    refer_to_a2: flatbuffers::Offset<super::NamespaceA::SecondTableInA>  /* = 0 */) -> flatbuffers::Offset<TableInC> {
  let mut builder = TableInCBuilder::new(_fbb);
  builder.add_refer_to_a2(refer_to_a2);
  builder.add_refer_to_a1(refer_to_a1);
  builder.finish()
}

#[inline]
fn TableInCTypeTable() -> /*&mut?*/flatbuffers::TypeTable {
  return flatbuffers::TypeTable{};
  /* disable type table for now
  static flatbuffers::TypeCode type_codes[] = {
    { flatbuffers::ET_SEQUENCE, 0, 0 },
    { flatbuffers::ET_SEQUENCE, 0, 1 }
  };
  static flatbuffers::TypeFunction type_refs[] = {
    NamespaceA::TableInFirstNSTypeTable,
    NamespaceA::SecondTableInATypeTable
  };
  static const char *names[] = {
    "refer_to_a1",
    "refer_to_a2"
  };
  static flatbuffers::TypeTable tt = {
    flatbuffers::ST_TABLE, 2, type_codes, type_refs, nullptr, names
  };
  return &tt;
  */
}

}  // pub mod NamespaceC

