mod builder;
mod endian_scalar;
mod follow;
mod primitives;
mod push;
mod table;
mod vector;
mod vtable;
mod vtable_writer;

pub use builder::{FlatBufferBuilder, field_index_to_field_offset};
pub use endian_scalar::{EndianScalar, emplace_scalar, read_scalar, read_scalar_at};
pub use follow::{Follow, FollowStart};
pub use primitives::*;
pub use push::Push;
pub use table::{Table, buffer_has_identifier, get_root, get_size_prefixed_root};
pub use vector::{SafeSliceAccess, Vector, follow_cast_ref};

// TODO(rw): Split fill ops in builder into fill_small, fill_big like in C++.
