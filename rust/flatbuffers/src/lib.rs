pub mod builder;
pub use builder::*;

pub mod endian_scalar;
pub use endian_scalar::*;

pub mod follow;
pub use follow::*;

pub mod primitives;
pub use primitives::*;

pub mod push;
pub use push::*;

pub mod table;
pub use table::*;

pub mod vector;
pub use vector::*;

pub mod vtable;
pub use vtable::*;

pub mod vtable_writer;
pub use vtable_writer::*;

// TODO(rw): figure out better trait bounds for implementing Follow on
//           EndianScalar and GeneratedStruct
// TODO(rw): use macros to impl EndianScalar and as_slice on primitives
// TODO(rw): split fill ops in builder into fill_small, fill_big like in C++
