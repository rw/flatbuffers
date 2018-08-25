pub mod my_game {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::marker::PhantomData;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum InParentNamespaceOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct InParentNamespace<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for InParentNamespace<'a> {
    type Inner = InParentNamespace<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> InParentNamespace<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        InParentNamespace {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        _args: &'y InParentNamespaceArgs<'y>) -> flatbuffers::Offset<InParentNamespace<'x>> {
      let mut builder = InParentNamespaceBuilder::new(_fbb);
      builder.finish()
    }

}

pub struct InParentNamespaceArgs<'a> {
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for InParentNamespaceArgs<'a> {
    fn default() -> Self {
        InParentNamespaceArgs {
            _phantom: PhantomData,
        }
    }
}
pub struct InParentNamespaceBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> InParentNamespaceBuilder<'a, 'b> {
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> InParentNamespaceBuilder<'a, 'b> {
    let start = _fbb.start_table();
    InParentNamespaceBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<InParentNamespace<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

pub mod example_2 {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::marker::PhantomData;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum MonsterOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Monster<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
    type Inner = Monster<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> Monster<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Monster {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        _args: &'y MonsterArgs<'y>) -> flatbuffers::Offset<Monster<'x>> {
      let mut builder = MonsterBuilder::new(_fbb);
      builder.finish()
    }

}

pub struct MonsterArgs<'a> {
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for MonsterArgs<'a> {
    fn default() -> Self {
        MonsterArgs {
            _phantom: PhantomData,
        }
    }
}
pub struct MonsterBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> MonsterBuilder<'a, 'b> {
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MonsterBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MonsterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<Monster<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

}  // pub mod Example2

pub mod example {
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
pub enum Color {
  Red = 1,
  Green = 2,
  Blue = 8
}

const ENUM_MIN_COLOR: i8 = 1;
const ENUM_MAX_COLOR: i8 = 8;

impl<'a> flatbuffers::Follow<'a> for Color {
  type Inner = Self;
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Color {
  fn to_little_endian(self) -> Self {
    let n = i8::to_le(self as i8);
    let p = &n as *const i8 as *const Color;
    unsafe { *p }
  }
  fn from_little_endian(self) -> Self {
    let n = i8::from_le(self as i8);
    let p = &n as *const i8 as *const Color;
    unsafe { *p }
  }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_COLOR:[Color; 3] = [
  Color::Red,
  Color::Green,
  Color::Blue
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_COLOR:[&'static str; 8] = [
    "Red",
    "Green",
    "",
    "",
    "",
    "",
    "",
    "Blue"
];

pub fn enum_name_color(e: Color) -> &'static str {
  let index: usize = e as usize - Color::Red as usize;
  ENUM_NAMES_COLOR[index]
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorBig {
  Red = 0,
  Green = 1
}

const ENUM_MIN_COLOR_BIG: u8 = 0;
const ENUM_MAX_COLOR_BIG: u8 = 1;

impl<'a> flatbuffers::Follow<'a> for ColorBig {
  type Inner = Self;
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for ColorBig {
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const ColorBig;
    unsafe { *p }
  }
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const ColorBig;
    unsafe { *p }
  }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_COLOR_BIG:[ColorBig; 2] = [
  ColorBig::Red,
  ColorBig::Green
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_COLOR_BIG:[&'static str; 2] = [
    "Red",
    "Green"
];

pub fn enum_name_color_big(e: ColorBig) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_COLOR_BIG[index]
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Any {
  NONE = 0,
  Monster = 1,
  TestSimpleTableWithEnum = 2,
  MyGame_Example2_Monster = 3
}

const ENUM_MIN_ANY: u8 = 0;
const ENUM_MAX_ANY: u8 = 3;

impl<'a> flatbuffers::Follow<'a> for Any {
  type Inner = Self;
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Any {
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Any;
    unsafe { *p }
  }
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Any;
    unsafe { *p }
  }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_ANY:[Any; 4] = [
  Any::NONE,
  Any::Monster,
  Any::TestSimpleTableWithEnum,
  Any::MyGame_Example2_Monster
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_ANY:[&'static str; 4] = [
    "NONE",
    "Monster",
    "TestSimpleTableWithEnum",
    "MyGame_Example2_Monster"
];

pub fn enum_name_any(e: Any) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_ANY[index]
}

pub struct AnyUnionTableOffset {}
// Size STRUCT_BYTE_SIZE, aligned to 2
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Test {
  a_: i16,
  b_: i8,
  padding0__: u8,
} // pub struct Test
impl flatbuffers::GeneratedStruct for Test {}

impl Test {
  pub fn new<'a>(_a: i16, _b: i8) -> Self {
    Test {
      a_: _a.to_little_endian(),
      b_: _b.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn a<'a>(&'a self) -> i16 {
    self.a_.from_little_endian()
  }
  pub fn b<'a>(&'a self) -> i8 {
    self.b_.from_little_endian()
  }
}

// Size STRUCT_BYTE_SIZE, aligned to 16
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
  x_: f32,
  y_: f32,
  z_: f32,
  padding0__: u32,
  test1_: f64,
  test2_: Color,
  padding1__: u8,
  test3_: Test/* foo */,
  padding2__: u16,
} // pub struct Vec3
impl flatbuffers::GeneratedStruct for Vec3 {}

impl Vec3 {
  pub fn new<'a>(_x: f32, _y: f32, _z: f32, _test1: f64, _test2: Color, _test3: &'a Test/* foo */) -> Self {
    Vec3 {
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),
      z_: _z.to_little_endian(),
      test1_: _test1.to_little_endian(),
      test2_: _test2.to_little_endian(),
      test3_: *_test3,

      padding0__: 0,
      padding1__: 0,
      padding2__: 0,
    }
  }
  pub fn x<'a>(&'a self) -> f32 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> f32 {
    self.y_.from_little_endian()
  }
  pub fn z<'a>(&'a self) -> f32 {
    self.z_.from_little_endian()
  }
  pub fn test1<'a>(&'a self) -> f64 {
    self.test1_.from_little_endian()
  }
  pub fn test2<'a>(&'a self) -> Color {
    self.test2_.from_little_endian()
  }
  pub fn test3<'a>(&'a self) -> &'a Test {
    &self.test3_
  }
}

// Size STRUCT_BYTE_SIZE, aligned to 4
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ability {
  id_: u32,
  distance_: u32,
} // pub struct Ability
impl flatbuffers::GeneratedStruct for Ability {}

impl Ability {
  pub fn new<'a>(_id: u32, _distance: u32) -> Self {
    Ability {
      id_: _id.to_little_endian(),
      distance_: _distance.to_little_endian(),

    }
  }
  pub fn id<'a>(&'a self) -> u32 {
    self.id_.from_little_endian()
  }
  pub fn distance<'a>(&'a self) -> u32 {
    self.distance_.from_little_endian()
  }
}

pub enum TestSimpleTableWithEnumOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TestSimpleTableWithEnum<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for TestSimpleTableWithEnum<'a> {
    type Inner = TestSimpleTableWithEnum<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> TestSimpleTableWithEnum<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TestSimpleTableWithEnum {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y TestSimpleTableWithEnumArgs<'y>) -> flatbuffers::Offset<TestSimpleTableWithEnum<'x>> {
      let mut builder = TestSimpleTableWithEnumBuilder::new(_fbb);
      builder.add_color(args.color);
      builder.finish()
    }

    pub const VT_COLOR: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn color(&'a self) -> Color {
    self._tab.get::<Color>(TestSimpleTableWithEnum::VT_COLOR, Some(Color::Green)).unwrap()
  }
}

pub struct TestSimpleTableWithEnumArgs<'a> {
    pub color: Color,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TestSimpleTableWithEnumArgs<'a> {
    fn default() -> Self {
        TestSimpleTableWithEnumArgs {
            color: Color::Green,
            _phantom: PhantomData,
        }
    }
}
pub struct TestSimpleTableWithEnumBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> TestSimpleTableWithEnumBuilder<'a, 'b> {
  pub fn add_color(&mut self, color: Color) {
    self.fbb_.push_slot_scalar::<Color>(TestSimpleTableWithEnum::VT_COLOR, color, Color::Green);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TestSimpleTableWithEnumBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TestSimpleTableWithEnumBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<TestSimpleTableWithEnum<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

pub enum StatOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Stat<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for Stat<'a> {
    type Inner = Stat<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> Stat<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Stat {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y StatArgs<'y>) -> flatbuffers::Offset<Stat<'x>> {
      let mut builder = StatBuilder::new(_fbb);
      builder.add_val(args.val);
      if let Some(x) = args.id { builder.add_id(x); }
      builder.add_count(args.count);
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_VAL: flatbuffers::VOffsetT = 6;
    pub const VT_COUNT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn id(&'a self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Stat::VT_ID, None)
  }
  #[inline]
  pub fn val(&'a self) -> i64 {
    self._tab.get::<i64>(Stat::VT_VAL, Some(0)).unwrap()
  }
  #[inline]
  pub fn count(&'a self) -> u16 {
    self._tab.get::<u16>(Stat::VT_COUNT, Some(0)).unwrap()
  }
}

pub struct StatArgs<'a> {
    pub id: Option<flatbuffers::Offset<&'a  str>>,
    pub val: i64,
    pub count: u16,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for StatArgs<'a> {
    fn default() -> Self {
        StatArgs {
            id: None,
            val: 0,
            count: 0,
            _phantom: PhantomData,
        }
    }
}
pub struct StatBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> StatBuilder<'a, 'b> {
  pub fn add_id(&mut self, id: flatbuffers::Offset<&'b  str>) {
    self.fbb_.push_slot_offset_relative(Stat::VT_ID, id);
  }
  pub fn add_val(&mut self, val: i64) {
    self.fbb_.push_slot_scalar::<i64>(Stat::VT_VAL, val, 0);
  }
  pub fn add_count(&mut self, count: u16) {
    self.fbb_.push_slot_scalar::<u16>(Stat::VT_COUNT, count, 0);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> StatBuilder<'a, 'b> {
    let start = _fbb.start_table();
    StatBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<Stat<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

pub enum ReferrableOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Referrable<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for Referrable<'a> {
    type Inner = Referrable<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> Referrable<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Referrable {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y ReferrableArgs<'y>) -> flatbuffers::Offset<Referrable<'x>> {
      let mut builder = ReferrableBuilder::new(_fbb);
      builder.add_id(args.id);
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn id(&'a self) -> u64 {
    self._tab.get::<u64>(Referrable::VT_ID, Some(0)).unwrap()
  }
}

pub struct ReferrableArgs<'a> {
    pub id: u64,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for ReferrableArgs<'a> {
    fn default() -> Self {
        ReferrableArgs {
            id: 0,
            _phantom: PhantomData,
        }
    }
}
pub struct ReferrableBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> ReferrableBuilder<'a, 'b> {
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot_scalar::<u64>(Referrable::VT_ID, id, 0);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ReferrableBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ReferrableBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<Referrable<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

/// an example documentation comment: monster object
pub enum MonsterOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Monster<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
    type Inner = Monster<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> Monster<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Monster {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y MonsterArgs<'y>) -> flatbuffers::Offset<Monster<'x>> {
      let mut builder = MonsterBuilder::new(_fbb);
      builder.add_non_owning_reference(args.non_owning_reference);
      builder.add_co_owning_reference(args.co_owning_reference);
      builder.add_single_weak_reference(args.single_weak_reference);
      builder.add_testhashu64_fnv1a(args.testhashu64_fnv1a);
      builder.add_testhashs64_fnv1a(args.testhashs64_fnv1a);
      builder.add_testhashu64_fnv1(args.testhashu64_fnv1);
      builder.add_testhashs64_fnv1(args.testhashs64_fnv1);
      if let Some(x) = args.vector_of_non_owning_references { builder.add_vector_of_non_owning_references(x); }
      if let Some(x) = args.vector_of_co_owning_references { builder.add_vector_of_co_owning_references(x); }
      if let Some(x) = args.vector_of_strong_referrables { builder.add_vector_of_strong_referrables(x); }
      if let Some(x) = args.vector_of_weak_references { builder.add_vector_of_weak_references(x); }
      if let Some(x) = args.vector_of_referrables { builder.add_vector_of_referrables(x); }
      if let Some(x) = args.parent_namespace_test { builder.add_parent_namespace_test(x); }
      if let Some(x) = args.vector_of_doubles { builder.add_vector_of_doubles(x); }
      if let Some(x) = args.vector_of_longs { builder.add_vector_of_longs(x); }
      if let Some(x) = args.test5 { builder.add_test5(x); }
      if let Some(x) = args.flex { builder.add_flex(x); }
      if let Some(x) = args.testarrayofsortedstruct { builder.add_testarrayofsortedstruct(x); }
      if let Some(x) = args.testarrayofstring2 { builder.add_testarrayofstring2(x); }
      builder.add_testf3(args.testf3);
      builder.add_testf2(args.testf2);
      builder.add_testf(args.testf);
      if let Some(x) = args.testarrayofbools { builder.add_testarrayofbools(x); }
      builder.add_testhashu32_fnv1a(args.testhashu32_fnv1a);
      builder.add_testhashs32_fnv1a(args.testhashs32_fnv1a);
      builder.add_testhashu32_fnv1(args.testhashu32_fnv1);
      builder.add_testhashs32_fnv1(args.testhashs32_fnv1);
      if let Some(x) = args.testempty { builder.add_testempty(x); }
      if let Some(x) = args.testnestedflatbuffer { builder.add_testnestedflatbuffer(x); }
      if let Some(x) = args.enemy { builder.add_enemy(x); }
      if let Some(x) = args.testarrayoftables { builder.add_testarrayoftables(x); }
      if let Some(x) = args.testarrayofstring { builder.add_testarrayofstring(x); }
      if let Some(x) = args.test4 { builder.add_test4(x); }
      if let Some(x) = args.test { builder.add_test(x); }
      if let Some(x) = args.inventory { builder.add_inventory(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      if let Some(x) = args.pos { builder.add_pos(x); }
      builder.add_hp(args.hp);
      builder.add_mana(args.mana);
      builder.add_testbool(args.testbool);
      builder.add_test_type(args.test_type);
      builder.add_color(args.color);
      builder.finish()
    }

    pub const VT_POS: flatbuffers::VOffsetT = 4;
    pub const VT_MANA: flatbuffers::VOffsetT = 6;
    pub const VT_HP: flatbuffers::VOffsetT = 8;
    pub const VT_NAME: flatbuffers::VOffsetT = 10;
    pub const VT_INVENTORY: flatbuffers::VOffsetT = 14;
    pub const VT_COLOR: flatbuffers::VOffsetT = 16;
    pub const VT_TEST_TYPE: flatbuffers::VOffsetT = 18;
    pub const VT_TEST: flatbuffers::VOffsetT = 20;
    pub const VT_TEST4: flatbuffers::VOffsetT = 22;
    pub const VT_TESTARRAYOFSTRING: flatbuffers::VOffsetT = 24;
    pub const VT_TESTARRAYOFTABLES: flatbuffers::VOffsetT = 26;
    pub const VT_ENEMY: flatbuffers::VOffsetT = 28;
    pub const VT_TESTNESTEDFLATBUFFER: flatbuffers::VOffsetT = 30;
    pub const VT_TESTEMPTY: flatbuffers::VOffsetT = 32;
    pub const VT_TESTBOOL: flatbuffers::VOffsetT = 34;
    pub const VT_TESTHASHS32_FNV1: flatbuffers::VOffsetT = 36;
    pub const VT_TESTHASHU32_FNV1: flatbuffers::VOffsetT = 38;
    pub const VT_TESTHASHS64_FNV1: flatbuffers::VOffsetT = 40;
    pub const VT_TESTHASHU64_FNV1: flatbuffers::VOffsetT = 42;
    pub const VT_TESTHASHS32_FNV1A: flatbuffers::VOffsetT = 44;
    pub const VT_TESTHASHU32_FNV1A: flatbuffers::VOffsetT = 46;
    pub const VT_TESTHASHS64_FNV1A: flatbuffers::VOffsetT = 48;
    pub const VT_TESTHASHU64_FNV1A: flatbuffers::VOffsetT = 50;
    pub const VT_TESTARRAYOFBOOLS: flatbuffers::VOffsetT = 52;
    pub const VT_TESTF: flatbuffers::VOffsetT = 54;
    pub const VT_TESTF2: flatbuffers::VOffsetT = 56;
    pub const VT_TESTF3: flatbuffers::VOffsetT = 58;
    pub const VT_TESTARRAYOFSTRING2: flatbuffers::VOffsetT = 60;
    pub const VT_TESTARRAYOFSORTEDSTRUCT: flatbuffers::VOffsetT = 62;
    pub const VT_FLEX: flatbuffers::VOffsetT = 64;
    pub const VT_TEST5: flatbuffers::VOffsetT = 66;
    pub const VT_VECTOR_OF_LONGS: flatbuffers::VOffsetT = 68;
    pub const VT_VECTOR_OF_DOUBLES: flatbuffers::VOffsetT = 70;
    pub const VT_PARENT_NAMESPACE_TEST: flatbuffers::VOffsetT = 72;
    pub const VT_VECTOR_OF_REFERRABLES: flatbuffers::VOffsetT = 74;
    pub const VT_SINGLE_WEAK_REFERENCE: flatbuffers::VOffsetT = 76;
    pub const VT_VECTOR_OF_WEAK_REFERENCES: flatbuffers::VOffsetT = 78;
    pub const VT_VECTOR_OF_STRONG_REFERRABLES: flatbuffers::VOffsetT = 80;
    pub const VT_CO_OWNING_REFERENCE: flatbuffers::VOffsetT = 82;
    pub const VT_VECTOR_OF_CO_OWNING_REFERENCES: flatbuffers::VOffsetT = 84;
    pub const VT_NON_OWNING_REFERENCE: flatbuffers::VOffsetT = 86;
    pub const VT_VECTOR_OF_NON_OWNING_REFERENCES: flatbuffers::VOffsetT = 88;

  #[inline]
  pub fn pos(&'a self) -> Option<&'a Vec3> {
    self._tab.get::<&'a Vec3>(Monster::VT_POS, None)
  }
  #[inline]
  pub fn mana(&'a self) -> i16 {
    self._tab.get::<i16>(Monster::VT_MANA, Some(150)).unwrap()
  }
  #[inline]
  pub fn hp(&'a self) -> i16 {
    self._tab.get::<i16>(Monster::VT_HP, Some(100)).unwrap()
  }
  #[inline]
  pub fn name(&'a self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Monster::VT_NAME, None)
  }
  #[inline]
  pub fn inventory(&'a self) -> Option<flatbuffers::Vector<'a, u8>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Monster::VT_INVENTORY, None)
  }
  #[inline]
  pub fn color(&'a self) -> Color {
    self._tab.get::<Color>(Monster::VT_COLOR, Some(Color::Blue)).unwrap()
  }
  #[inline]
  pub fn test_type(&'a self) -> Any {
    self._tab.get::<Any>(Monster::VT_TEST_TYPE, Some(Any::NONE)).unwrap()
  }
  #[inline]
  pub fn test(&'a self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Monster::VT_TEST, None)
  }
  #[inline]
  pub fn test4(&'a self) -> Option<&'a [Test]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::SliceOfGeneratedStruct<Test>>>(Monster::VT_TEST4, None)
  }
  #[inline]
  pub fn testarrayofstring(&'a self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(Monster::VT_TESTARRAYOFSTRING, None)
  }
  /// an example documentation comment: this will end up in the generated code
  /// multiline too
  #[inline]
  pub fn testarrayoftables(&'a self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Monster<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Monster<'a>>>>>(Monster::VT_TESTARRAYOFTABLES, None)
  }
  #[inline]
  pub fn enemy(&'a self) -> Option<Monster<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Monster<'a>>>(Monster::VT_ENEMY, None)
  }
  #[inline]
  pub fn testnestedflatbuffer(&'a self) -> Option<flatbuffers::Vector<'a, u8>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Monster::VT_TESTNESTEDFLATBUFFER, None)
  }
  pub fn testnestedflatbuffer_nested_flatbuffer(&'a self) -> Option<Monster<'a>> {
     match self.testnestedflatbuffer() {
         None => { None }
         Some(data) => {
             use self::flatbuffers::Follow;
             Some(<flatbuffers::ForwardsUOffset<Monster<'a>>>::follow(data.as_slice(), 0))
         },
     }
  }
  #[inline]
  pub fn testempty(&'a self) -> Option<Stat<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Stat<'a>>>(Monster::VT_TESTEMPTY, None)
  }
  #[inline]
  pub fn testbool(&'a self) -> bool {
    self._tab.get::<bool>(Monster::VT_TESTBOOL, Some(false)).unwrap()
  }
  #[inline]
  pub fn testhashs32_fnv1(&'a self) -> i32 {
    self._tab.get::<i32>(Monster::VT_TESTHASHS32_FNV1, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashu32_fnv1(&'a self) -> u32 {
    self._tab.get::<u32>(Monster::VT_TESTHASHU32_FNV1, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashs64_fnv1(&'a self) -> i64 {
    self._tab.get::<i64>(Monster::VT_TESTHASHS64_FNV1, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashu64_fnv1(&'a self) -> u64 {
    self._tab.get::<u64>(Monster::VT_TESTHASHU64_FNV1, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashs32_fnv1a(&'a self) -> i32 {
    self._tab.get::<i32>(Monster::VT_TESTHASHS32_FNV1A, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashu32_fnv1a(&'a self) -> u32 {
    self._tab.get::<u32>(Monster::VT_TESTHASHU32_FNV1A, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashs64_fnv1a(&'a self) -> i64 {
    self._tab.get::<i64>(Monster::VT_TESTHASHS64_FNV1A, Some(0)).unwrap()
  }
  #[inline]
  pub fn testhashu64_fnv1a(&'a self) -> u64 {
    self._tab.get::<u64>(Monster::VT_TESTHASHU64_FNV1A, Some(0)).unwrap()
  }
  #[inline]
  pub fn testarrayofbools(&'a self) -> Option<flatbuffers::Vector<'a, bool>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, bool>>>(Monster::VT_TESTARRAYOFBOOLS, None)
  }
  #[inline]
  pub fn testf(&'a self) -> f32 {
    self._tab.get::<f32>(Monster::VT_TESTF, Some(3.14159)).unwrap()
  }
  #[inline]
  pub fn testf2(&'a self) -> f32 {
    self._tab.get::<f32>(Monster::VT_TESTF2, Some(3.0)).unwrap()
  }
  #[inline]
  pub fn testf3(&'a self) -> f32 {
    self._tab.get::<f32>(Monster::VT_TESTF3, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn testarrayofstring2(&'a self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(Monster::VT_TESTARRAYOFSTRING2, None)
  }
  #[inline]
  pub fn testarrayofsortedstruct(&'a self) -> Option<&'a [Ability]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::SliceOfGeneratedStruct<Ability>>>(Monster::VT_TESTARRAYOFSORTEDSTRUCT, None)
  }
  #[inline]
  pub fn flex(&'a self) -> Option<flatbuffers::Vector<'a, u8>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Monster::VT_FLEX, None)
  }
  #[inline]
  pub fn test5(&'a self) -> Option<&'a [Test]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::SliceOfGeneratedStruct<Test>>>(Monster::VT_TEST5, None)
  }
  #[inline]
  pub fn vector_of_longs(&'a self) -> Option<flatbuffers::Vector<'a, i64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i64>>>(Monster::VT_VECTOR_OF_LONGS, None)
  }
  #[inline]
  pub fn vector_of_doubles(&'a self) -> Option<flatbuffers::Vector<'a, f64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f64>>>(Monster::VT_VECTOR_OF_DOUBLES, None)
  }
  #[inline]
  pub fn parent_namespace_test(&'a self) -> Option<super::InParentNamespace<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::InParentNamespace<'a>>>(Monster::VT_PARENT_NAMESPACE_TEST, None)
  }
  #[inline]
  pub fn vector_of_referrables(&'a self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Referrable<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Referrable<'a>>>>>(Monster::VT_VECTOR_OF_REFERRABLES, None)
  }
  #[inline]
  pub fn single_weak_reference(&'a self) -> u64 {
    self._tab.get::<u64>(Monster::VT_SINGLE_WEAK_REFERENCE, Some(0)).unwrap()
  }
  #[inline]
  pub fn vector_of_weak_references(&'a self) -> Option<flatbuffers::Vector<'a, u64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(Monster::VT_VECTOR_OF_WEAK_REFERENCES, None)
  }
  #[inline]
  pub fn vector_of_strong_referrables(&'a self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Referrable<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Referrable<'a>>>>>(Monster::VT_VECTOR_OF_STRONG_REFERRABLES, None)
  }
  #[inline]
  pub fn co_owning_reference(&'a self) -> u64 {
    self._tab.get::<u64>(Monster::VT_CO_OWNING_REFERENCE, Some(0)).unwrap()
  }
  #[inline]
  pub fn vector_of_co_owning_references(&'a self) -> Option<flatbuffers::Vector<'a, u64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(Monster::VT_VECTOR_OF_CO_OWNING_REFERENCES, None)
  }
  #[inline]
  pub fn non_owning_reference(&'a self) -> u64 {
    self._tab.get::<u64>(Monster::VT_NON_OWNING_REFERENCE, Some(0)).unwrap()
  }
  #[inline]
  pub fn vector_of_non_owning_references(&'a self) -> Option<flatbuffers::Vector<'a, u64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(Monster::VT_VECTOR_OF_NON_OWNING_REFERENCES, None)
  }
}

//TODO: inject these functions into impl for type
//#[inline]
//fn Monster_MEMBER_test_as_X_Monster_X() -> &Monster {
//  return test_as_Monster();
//}
//
//TODO: inject these functions into impl for type
//#[inline]
//fn Monster_MEMBER_test_as_X_TestSimpleTableWithEnum_X() -> &TestSimpleTableWithEnum {
//  return test_as_TestSimpleTableWithEnum();
//}
//
//TODO: inject these functions into impl for type
//#[inline]
//fn Monster_MEMBER_test_as_X_super::example_2::Monster_X() -> &super::example_2::Monster {
//  return test_as_MyGame_Example2_Monster();
//}
//
pub struct MonsterArgs<'a> {
    pub pos: Option<&'a  Vec3/* foo */>,
    pub mana: i16,
    pub hp: i16,
    pub name: Option<flatbuffers::Offset<&'a  str>>,
    pub inventory: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  u8>>>,
    pub color: Color,
    pub test_type: Any,
    pub test: Option<flatbuffers::Offset<flatbuffers::UnionMarker>>,
    pub test4: Option<flatbuffers::Offset<flatbuffers::Vector<'a , Test>>>,
    pub testarrayofstring: Option<flatbuffers::Offset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
    pub testarrayoftables: Option<flatbuffers::Offset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Monster<'a >>>>>,
    pub enemy: Option<flatbuffers::Offset<Monster<'a >>>,
    pub testnestedflatbuffer: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  u8>>>,
    pub testempty: Option<flatbuffers::Offset<Stat<'a >>>,
    pub testbool: bool,
    pub testhashs32_fnv1: i32,
    pub testhashu32_fnv1: u32,
    pub testhashs64_fnv1: i64,
    pub testhashu64_fnv1: u64,
    pub testhashs32_fnv1a: i32,
    pub testhashu32_fnv1a: u32,
    pub testhashs64_fnv1a: i64,
    pub testhashu64_fnv1a: u64,
    pub testarrayofbools: Option<flatbuffers::Offset<flatbuffers::Vector<'a , bool>>>,
    pub testf: f32,
    pub testf2: f32,
    pub testf3: f32,
    pub testarrayofstring2: Option<flatbuffers::Offset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
    pub testarrayofsortedstruct: Option<flatbuffers::Offset<flatbuffers::Vector<'a , Ability>>>,
    pub flex: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  u8>>>,
    pub test5: Option<flatbuffers::Offset<flatbuffers::Vector<'a , Test>>>,
    pub vector_of_longs: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  i64>>>,
    pub vector_of_doubles: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  f64>>>,
    pub parent_namespace_test: Option<flatbuffers::Offset<super::InParentNamespace<'a >>>,
    pub vector_of_referrables: Option<flatbuffers::Offset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Referrable<'a >>>>>,
    pub single_weak_reference: u64,
    pub vector_of_weak_references: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  u64>>>,
    pub vector_of_strong_referrables: Option<flatbuffers::Offset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Referrable<'a >>>>>,
    pub co_owning_reference: u64,
    pub vector_of_co_owning_references: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  u64>>>,
    pub non_owning_reference: u64,
    pub vector_of_non_owning_references: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  u64>>>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for MonsterArgs<'a> {
    fn default() -> Self {
        MonsterArgs {
            pos: None,
            mana: 150,
            hp: 100,
 // required
            name: None,
            inventory: None,
            color: Color::Blue,
            test_type: Any::NONE,
            test: None,
            test4: None,
            testarrayofstring: None,
            testarrayoftables: None,
            enemy: None,
            testnestedflatbuffer: None,
            testempty: None,
            testbool: false,
            testhashs32_fnv1: 0,
            testhashu32_fnv1: 0,
            testhashs64_fnv1: 0,
            testhashu64_fnv1: 0,
            testhashs32_fnv1a: 0,
            testhashu32_fnv1a: 0,
            testhashs64_fnv1a: 0,
            testhashu64_fnv1a: 0,
            testarrayofbools: None,
            testf: 3.14159,
            testf2: 3.0,
            testf3: 0.0,
            testarrayofstring2: None,
            testarrayofsortedstruct: None,
            flex: None,
            test5: None,
            vector_of_longs: None,
            vector_of_doubles: None,
            parent_namespace_test: None,
            vector_of_referrables: None,
            single_weak_reference: 0,
            vector_of_weak_references: None,
            vector_of_strong_referrables: None,
            co_owning_reference: 0,
            vector_of_co_owning_references: None,
            non_owning_reference: 0,
            vector_of_non_owning_references: None,
            _phantom: PhantomData,
        }
    }
}
pub struct MonsterBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> MonsterBuilder<'a, 'b> {
  pub fn add_pos(&mut self, pos: &'b  Vec3) {
    self.fbb_.push_slot_struct::<Vec3/* foo */>(Monster::VT_POS, pos);
  }
  pub fn add_mana(&mut self, mana: i16) {
    self.fbb_.push_slot_scalar::<i16>(Monster::VT_MANA, mana, 150);
  }
  pub fn add_hp(&mut self, hp: i16) {
    self.fbb_.push_slot_scalar::<i16>(Monster::VT_HP, hp, 100);
  }
  pub fn add_name(&mut self, name: flatbuffers::Offset<&'b  str>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_NAME, name);
  }
  pub fn add_inventory(&mut self, inventory: flatbuffers::Offset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_INVENTORY, inventory);
  }
  pub fn add_color(&mut self, color: Color) {
    self.fbb_.push_slot_scalar::<Color>(Monster::VT_COLOR, color, Color::Blue);
  }
  pub fn add_test_type(&mut self, test_type: Any) {
    self.fbb_.push_slot_scalar::<Any>(Monster::VT_TEST_TYPE, test_type, Any::NONE);
  }
  pub fn add_test(&mut self, test: flatbuffers::Offset<flatbuffers::UnionMarker>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TEST, test);
  }
  pub fn add_test4(&mut self, test4: flatbuffers::Offset<flatbuffers::Vector<'b , Test>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TEST4, test4);
  }
  pub fn add_testarrayofstring(&mut self, testarrayofstring: flatbuffers::Offset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TESTARRAYOFSTRING, testarrayofstring);
  }
  pub fn add_testarrayoftables(&mut self, testarrayoftables: flatbuffers::Offset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Monster<'b >>>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TESTARRAYOFTABLES, testarrayoftables);
  }
  pub fn add_enemy(&mut self, enemy: flatbuffers::Offset<Monster<'b >>) {
    self.fbb_.push_slot_offset_relative::<Monster>(Monster::VT_ENEMY, enemy);
  }
  pub fn add_testnestedflatbuffer(&mut self, testnestedflatbuffer: flatbuffers::Offset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TESTNESTEDFLATBUFFER, testnestedflatbuffer);
  }
  pub fn add_testempty(&mut self, testempty: flatbuffers::Offset<Stat<'b >>) {
    self.fbb_.push_slot_offset_relative::<Stat>(Monster::VT_TESTEMPTY, testempty);
  }
  pub fn add_testbool(&mut self, testbool: bool) {
    self.fbb_.push_slot_scalar::<bool>(Monster::VT_TESTBOOL, testbool, false);
  }
  pub fn add_testhashs32_fnv1(&mut self, testhashs32_fnv1: i32) {
    self.fbb_.push_slot_scalar::<i32>(Monster::VT_TESTHASHS32_FNV1, testhashs32_fnv1, 0);
  }
  pub fn add_testhashu32_fnv1(&mut self, testhashu32_fnv1: u32) {
    self.fbb_.push_slot_scalar::<u32>(Monster::VT_TESTHASHU32_FNV1, testhashu32_fnv1, 0);
  }
  pub fn add_testhashs64_fnv1(&mut self, testhashs64_fnv1: i64) {
    self.fbb_.push_slot_scalar::<i64>(Monster::VT_TESTHASHS64_FNV1, testhashs64_fnv1, 0);
  }
  pub fn add_testhashu64_fnv1(&mut self, testhashu64_fnv1: u64) {
    self.fbb_.push_slot_scalar::<u64>(Monster::VT_TESTHASHU64_FNV1, testhashu64_fnv1, 0);
  }
  pub fn add_testhashs32_fnv1a(&mut self, testhashs32_fnv1a: i32) {
    self.fbb_.push_slot_scalar::<i32>(Monster::VT_TESTHASHS32_FNV1A, testhashs32_fnv1a, 0);
  }
  pub fn add_testhashu32_fnv1a(&mut self, testhashu32_fnv1a: u32) {
    self.fbb_.push_slot_scalar::<u32>(Monster::VT_TESTHASHU32_FNV1A, testhashu32_fnv1a, 0);
  }
  pub fn add_testhashs64_fnv1a(&mut self, testhashs64_fnv1a: i64) {
    self.fbb_.push_slot_scalar::<i64>(Monster::VT_TESTHASHS64_FNV1A, testhashs64_fnv1a, 0);
  }
  pub fn add_testhashu64_fnv1a(&mut self, testhashu64_fnv1a: u64) {
    self.fbb_.push_slot_scalar::<u64>(Monster::VT_TESTHASHU64_FNV1A, testhashu64_fnv1a, 0);
  }
  pub fn add_testarrayofbools(&mut self, testarrayofbools: flatbuffers::Offset<flatbuffers::Vector<'b , bool>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TESTARRAYOFBOOLS, testarrayofbools);
  }
  pub fn add_testf(&mut self, testf: f32) {
    self.fbb_.push_slot_scalar::<f32>(Monster::VT_TESTF, testf, 3.14159);
  }
  pub fn add_testf2(&mut self, testf2: f32) {
    self.fbb_.push_slot_scalar::<f32>(Monster::VT_TESTF2, testf2, 3.0);
  }
  pub fn add_testf3(&mut self, testf3: f32) {
    self.fbb_.push_slot_scalar::<f32>(Monster::VT_TESTF3, testf3, 0.0);
  }
  pub fn add_testarrayofstring2(&mut self, testarrayofstring2: flatbuffers::Offset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TESTARRAYOFSTRING2, testarrayofstring2);
  }
  pub fn add_testarrayofsortedstruct(&mut self, testarrayofsortedstruct: flatbuffers::Offset<flatbuffers::Vector<'b , Ability>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TESTARRAYOFSORTEDSTRUCT, testarrayofsortedstruct);
  }
  pub fn add_flex(&mut self, flex: flatbuffers::Offset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_FLEX, flex);
  }
  pub fn add_test5(&mut self, test5: flatbuffers::Offset<flatbuffers::Vector<'b , Test>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_TEST5, test5);
  }
  pub fn add_vector_of_longs(&mut self, vector_of_longs: flatbuffers::Offset<flatbuffers::Vector<'b , i64>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_LONGS, vector_of_longs);
  }
  pub fn add_vector_of_doubles(&mut self, vector_of_doubles: flatbuffers::Offset<flatbuffers::Vector<'b , f64>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_DOUBLES, vector_of_doubles);
  }
  pub fn add_parent_namespace_test(&mut self, parent_namespace_test: flatbuffers::Offset<super::InParentNamespace<'b >>) {
    self.fbb_.push_slot_offset_relative::<super::InParentNamespace>(Monster::VT_PARENT_NAMESPACE_TEST, parent_namespace_test);
  }
  pub fn add_vector_of_referrables(&mut self, vector_of_referrables: flatbuffers::Offset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Referrable<'b >>>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_REFERRABLES, vector_of_referrables);
  }
  pub fn add_single_weak_reference(&mut self, single_weak_reference: u64) {
    self.fbb_.push_slot_scalar::<u64>(Monster::VT_SINGLE_WEAK_REFERENCE, single_weak_reference, 0);
  }
  pub fn add_vector_of_weak_references(&mut self, vector_of_weak_references: flatbuffers::Offset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_WEAK_REFERENCES, vector_of_weak_references);
  }
  pub fn add_vector_of_strong_referrables(&mut self, vector_of_strong_referrables: flatbuffers::Offset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Referrable<'b >>>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_STRONG_REFERRABLES, vector_of_strong_referrables);
  }
  pub fn add_co_owning_reference(&mut self, co_owning_reference: u64) {
    self.fbb_.push_slot_scalar::<u64>(Monster::VT_CO_OWNING_REFERENCE, co_owning_reference, 0);
  }
  pub fn add_vector_of_co_owning_references(&mut self, vector_of_co_owning_references: flatbuffers::Offset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_CO_OWNING_REFERENCES, vector_of_co_owning_references);
  }
  pub fn add_non_owning_reference(&mut self, non_owning_reference: u64) {
    self.fbb_.push_slot_scalar::<u64>(Monster::VT_NON_OWNING_REFERENCE, non_owning_reference, 0);
  }
  pub fn add_vector_of_non_owning_references(&mut self, vector_of_non_owning_references: flatbuffers::Offset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_offset_relative(Monster::VT_VECTOR_OF_NON_OWNING_REFERENCES, vector_of_non_owning_references);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MonsterBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MonsterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<Monster<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Monster::VT_NAME,"name");
    flatbuffers::Offset::new(o.value())
  }
}

pub enum TypeAliasesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TypeAliases<'a> {
  pub _tab: flatbuffers::Table<'a>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> flatbuffers::Follow<'a> for TypeAliases<'a> {
    type Inner = TypeAliases<'a>;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf: buf, loc: loc }, _phantom: PhantomData }
    }
}

impl<'a> TypeAliases<'a> {
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TypeAliases {
            _tab: table,
            _phantom: PhantomData,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'x: 'y, 'y: 'z, 'z>(
        _fbb: &'z mut flatbuffers::FlatBufferBuilder<'x>,
        args: &'y TypeAliasesArgs<'y>) -> flatbuffers::Offset<TypeAliases<'x>> {
      let mut builder = TypeAliasesBuilder::new(_fbb);
      builder.add_f64_(args.f64_);
      builder.add_u64_(args.u64_);
      builder.add_i64_(args.i64_);
      if let Some(x) = args.vf64 { builder.add_vf64(x); }
      if let Some(x) = args.v8 { builder.add_v8(x); }
      builder.add_f32_(args.f32_);
      builder.add_u32_(args.u32_);
      builder.add_i32_(args.i32_);
      builder.add_u16_(args.u16_);
      builder.add_i16_(args.i16_);
      builder.add_u8_(args.u8_);
      builder.add_i8_(args.i8_);
      builder.finish()
    }

    pub const VT_I8_: flatbuffers::VOffsetT = 4;
    pub const VT_U8_: flatbuffers::VOffsetT = 6;
    pub const VT_I16_: flatbuffers::VOffsetT = 8;
    pub const VT_U16_: flatbuffers::VOffsetT = 10;
    pub const VT_I32_: flatbuffers::VOffsetT = 12;
    pub const VT_U32_: flatbuffers::VOffsetT = 14;
    pub const VT_I64_: flatbuffers::VOffsetT = 16;
    pub const VT_U64_: flatbuffers::VOffsetT = 18;
    pub const VT_F32_: flatbuffers::VOffsetT = 20;
    pub const VT_F64_: flatbuffers::VOffsetT = 22;
    pub const VT_V8: flatbuffers::VOffsetT = 24;
    pub const VT_VF64: flatbuffers::VOffsetT = 26;

  #[inline]
  pub fn i8_(&'a self) -> i8 {
    self._tab.get::<i8>(TypeAliases::VT_I8_, Some(0)).unwrap()
  }
  #[inline]
  pub fn u8_(&'a self) -> u8 {
    self._tab.get::<u8>(TypeAliases::VT_U8_, Some(0)).unwrap()
  }
  #[inline]
  pub fn i16_(&'a self) -> i16 {
    self._tab.get::<i16>(TypeAliases::VT_I16_, Some(0)).unwrap()
  }
  #[inline]
  pub fn u16_(&'a self) -> u16 {
    self._tab.get::<u16>(TypeAliases::VT_U16_, Some(0)).unwrap()
  }
  #[inline]
  pub fn i32_(&'a self) -> i32 {
    self._tab.get::<i32>(TypeAliases::VT_I32_, Some(0)).unwrap()
  }
  #[inline]
  pub fn u32_(&'a self) -> u32 {
    self._tab.get::<u32>(TypeAliases::VT_U32_, Some(0)).unwrap()
  }
  #[inline]
  pub fn i64_(&'a self) -> i64 {
    self._tab.get::<i64>(TypeAliases::VT_I64_, Some(0)).unwrap()
  }
  #[inline]
  pub fn u64_(&'a self) -> u64 {
    self._tab.get::<u64>(TypeAliases::VT_U64_, Some(0)).unwrap()
  }
  #[inline]
  pub fn f32_(&'a self) -> f32 {
    self._tab.get::<f32>(TypeAliases::VT_F32_, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn f64_(&'a self) -> f64 {
    self._tab.get::<f64>(TypeAliases::VT_F64_, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn v8(&'a self) -> Option<flatbuffers::Vector<'a, i8>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i8>>>(TypeAliases::VT_V8, None)
  }
  #[inline]
  pub fn vf64(&'a self) -> Option<flatbuffers::Vector<'a, f64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f64>>>(TypeAliases::VT_VF64, None)
  }
}

pub struct TypeAliasesArgs<'a> {
    pub i8_: i8,
    pub u8_: u8,
    pub i16_: i16,
    pub u16_: u16,
    pub i32_: i32,
    pub u32_: u32,
    pub i64_: i64,
    pub u64_: u64,
    pub f32_: f32,
    pub f64_: f64,
    pub v8: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  i8>>>,
    pub vf64: Option<flatbuffers::Offset<flatbuffers::Vector<'a ,  f64>>>,
    pub _phantom: PhantomData<&'a ()>, // pub for default trait
}
impl<'a> Default for TypeAliasesArgs<'a> {
    fn default() -> Self {
        TypeAliasesArgs {
            i8_: 0,
            u8_: 0,
            i16_: 0,
            u16_: 0,
            i32_: 0,
            u32_: 0,
            i64_: 0,
            u64_: 0,
            f32_: 0.0,
            f64_: 0.0,
            v8: None,
            vf64: None,
            _phantom: PhantomData,
        }
    }
}
pub struct TypeAliasesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::Offset<flatbuffers::TableOffset>,
}
impl<'a: 'b, 'b> TypeAliasesBuilder<'a, 'b> {
  pub fn add_i8_(&mut self, i8_: i8) {
    self.fbb_.push_slot_scalar::<i8>(TypeAliases::VT_I8_, i8_, 0);
  }
  pub fn add_u8_(&mut self, u8_: u8) {
    self.fbb_.push_slot_scalar::<u8>(TypeAliases::VT_U8_, u8_, 0);
  }
  pub fn add_i16_(&mut self, i16_: i16) {
    self.fbb_.push_slot_scalar::<i16>(TypeAliases::VT_I16_, i16_, 0);
  }
  pub fn add_u16_(&mut self, u16_: u16) {
    self.fbb_.push_slot_scalar::<u16>(TypeAliases::VT_U16_, u16_, 0);
  }
  pub fn add_i32_(&mut self, i32_: i32) {
    self.fbb_.push_slot_scalar::<i32>(TypeAliases::VT_I32_, i32_, 0);
  }
  pub fn add_u32_(&mut self, u32_: u32) {
    self.fbb_.push_slot_scalar::<u32>(TypeAliases::VT_U32_, u32_, 0);
  }
  pub fn add_i64_(&mut self, i64_: i64) {
    self.fbb_.push_slot_scalar::<i64>(TypeAliases::VT_I64_, i64_, 0);
  }
  pub fn add_u64_(&mut self, u64_: u64) {
    self.fbb_.push_slot_scalar::<u64>(TypeAliases::VT_U64_, u64_, 0);
  }
  pub fn add_f32_(&mut self, f32_: f32) {
    self.fbb_.push_slot_scalar::<f32>(TypeAliases::VT_F32_, f32_, 0.0);
  }
  pub fn add_f64_(&mut self, f64_: f64) {
    self.fbb_.push_slot_scalar::<f64>(TypeAliases::VT_F64_, f64_, 0.0);
  }
  pub fn add_v8(&mut self, v8: flatbuffers::Offset<flatbuffers::Vector<'b , i8>>) {
    self.fbb_.push_slot_offset_relative(TypeAliases::VT_V8, v8);
  }
  pub fn add_vf64(&mut self, vf64: flatbuffers::Offset<flatbuffers::Vector<'b , f64>>) {
    self.fbb_.push_slot_offset_relative(TypeAliases::VT_VF64, vf64);
  }
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TypeAliasesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TypeAliasesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  pub fn finish(self) -> flatbuffers::Offset<TypeAliases<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::Offset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_monster<'a>(buf: &'a [u8]) -> Monster<'a> {
  flatbuffers::get_root::<Monster<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_monster<'a>(buf: &'a [u8]) -> Monster<'a> {
  flatbuffers::get_size_prefixed_root::<Monster<'a>>(buf)
}

pub const MONSTER_IDENTIFIER: &'static str = "MONS";

#[inline]
pub fn monster_buffer_has_identifier(buf: &[u8]) -> bool {
  return flatbuffers::buffer_has_identifier(buf, MONSTER_IDENTIFIER, false);
}

#[inline]
pub fn monster_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  return flatbuffers::buffer_has_identifier(buf, MONSTER_IDENTIFIER, true);
}

pub const MONSTER_EXTENSION: &'static str = "mon";

#[inline]
pub fn finish_monster_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::Offset<Monster<'a>>) {
  fbb.finish(root, Some(MONSTER_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_monster_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::Offset<Monster<'a>>) {
  fbb.finish_size_prefixed(root, Some(MONSTER_IDENTIFIER));
}
}  // pub mod Example
}  // pub mod MyGame

