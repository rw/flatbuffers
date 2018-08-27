extern crate smallvec;

use std::cmp::max;
use std::marker::PhantomData;
use std::mem::size_of;
use std::slice::from_raw_parts;

pub use endian_scalar::{EndianScalar, read_scalar, emplace_scalar};
pub use primitives::*;
pub use table::*;
pub use vtable::*;
use vtable_writer::*;
pub use vector::*;

pub struct AlignParams {
    len: usize,
    alignment: usize,
}

pub trait Push: Sized {
    type Output;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], _rest: &'a [u8]);
    fn size(&self) -> usize {
        size_of::<Self>()
    }
    fn align_params(&self) -> AlignParams {
        AlignParams{len: self.size(), alignment: self.size()}
    }
}


pub fn pushable_method_struct_do_write<'a, T: Sized + 'a>(x: &'a T, dst: &'a mut [u8], _rest: &'a [u8]) {
    let sz = ::std::mem::size_of::<T>();
    debug_assert_eq!(sz, dst.len());
    let src = unsafe {
        ::std::slice::from_raw_parts(x as *const T as *const u8, sz)
    };
    dst.copy_from_slice(src);
}

impl<'b> Push for &'b [u8] {
    type Output = Vector<'b, u8>;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], _rest: &'a [u8]) {
        let l = self.len() as UOffsetT;
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l);
        dst[SIZE_UOFFSET..].copy_from_slice(self);
    }
    fn size(&self) -> usize {
        self.len() + SIZE_UOFFSET
    }
    fn align_params(&self) -> AlignParams {
        AlignParams{len: self.size(), alignment: SIZE_UOFFSET}
    }
}

struct ZeroTerminatedByteSlice<'a>(&'a [u8]);
impl<'b> Push for ZeroTerminatedByteSlice<'b> {
    type Output = Vector<'b, u8>;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], _rest: &'a [u8]) {
        let data = self.0;
        let l = data.len();
        emplace_scalar::<UOffsetT>(&mut dst[..SIZE_UOFFSET], l as UOffsetT);
        dst[SIZE_UOFFSET..SIZE_UOFFSET+l].copy_from_slice(data);
    }
    fn size(&self) -> usize {
        SIZE_UOFFSET + self.0.len() + 1
    }
    fn align_params(&self) -> AlignParams {
        AlignParams{len: self.size(), alignment: SIZE_UOFFSET}
    }
}

#[macro_export]
macro_rules! impl_pushable_method_for_endian_scalar {
    ($ty:ident) => (
        impl Push for $ty {
            type Output = $ty;
            fn do_write<'a>(&'a self, dst: &'a mut [u8], _rest: &'a [u8]) {
                emplace_scalar::<$ty>(dst, *self);
            }
        }
    )
}
impl_pushable_method_for_endian_scalar!(bool);
impl_pushable_method_for_endian_scalar!(u8);
impl_pushable_method_for_endian_scalar!(i8);
impl_pushable_method_for_endian_scalar!(u16);
impl_pushable_method_for_endian_scalar!(i16);
impl_pushable_method_for_endian_scalar!(u32);
impl_pushable_method_for_endian_scalar!(i32);
impl_pushable_method_for_endian_scalar!(u64);
impl_pushable_method_for_endian_scalar!(i64);
impl_pushable_method_for_endian_scalar!(f32);
impl_pushable_method_for_endian_scalar!(f64);

#[macro_export]
macro_rules! impl_pushable_method_for_struct_reference {
    ($ty:ident) => (
        impl<'b> flatbuffers::Push for &'b $ty {
           type Output = $ty;
           fn do_write<'a>(&'a self, dst: &'a mut [u8], _rest: &'a [u8]) {
               let sz = ::std::mem::size_of::<$ty>();
               debug_assert_eq!(sz, dst.len());

               let src = unsafe {
                   ::std::slice::from_raw_parts(*self as *const $ty as *const u8, sz)
               };
               dst.copy_from_slice(src);
           }
           fn size(&self) -> usize {
               ::std::mem::size_of::<$ty>()
           }
        }
    )
}

impl<T> Push for Offset<T> {
    type Output = ForwardsUOffset<T>;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], rest: &'a [u8]) {
        debug_assert_eq!(dst.len(), SIZE_UOFFSET);
        let n = (SIZE_UOFFSET + rest.len() - self.value() as usize) as UOffsetT;
        emplace_scalar::<UOffsetT>(dst, n);
    }
}
impl<T> Push for ForwardsUOffset<T> {
    type Output = Self;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], rest: &'a [u8]) {
        self.value().do_write(dst, rest);
    }
}
impl<T> Push for ForwardsVOffset<T> {
    type Output = Self;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], rest: &'a [u8]) {
        self.value().do_write(dst, rest);
    }
}
impl<T> Push for BackwardsSOffset<T> {
    type Output = Self;
    fn do_write<'a>(&'a self, dst: &'a mut [u8], rest: &'a [u8]) {
        self.value().do_write(dst, rest);
    }
}

#[derive(Clone, Copy, Debug)]
struct FieldLoc {
    off: UOffsetT,
    id: VOffsetT,
}
pub struct FlatBufferBuilder<'fbb> {
    pub owned_buf: Vec<u8>,
    pub cur_idx: usize,

    field_locs: Vec<FieldLoc>,
    written_vtable_revpos: Vec<UOffsetT>,

    nested: bool,
    finished: bool,

    min_align: usize,
    max_voffset: VOffsetT,

    _phantom: PhantomData<&'fbb ()>,
}
impl<'fbb> FlatBufferBuilder<'fbb> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }
    pub fn new_with_capacity(size: usize) -> Self {
        FlatBufferBuilder {
            owned_buf: vec![0u8; size],
            cur_idx: size,

            field_locs: Vec::new(),
            written_vtable_revpos: Vec::new(),

            nested: false,
            finished: false,

            min_align: 0,
            max_voffset: 0,

            _phantom: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        self.owned_buf.clear();
	let cap = self.owned_buf.capacity();
        unsafe {
	    self.owned_buf.set_len(cap);
	}
        self.cur_idx = self.owned_buf.len();

        self.written_vtable_revpos.clear();

        self.nested = false;
        self.finished = false;

        self.min_align = 0;
        self.max_voffset = 0;
    }

    pub fn num_written_vtables(&self) -> usize {
        self.written_vtable_revpos.len()
    }
    pub fn get_active_buf_slice<'a>(&'a self) -> &'a [u8] {
        &self.owned_buf[self.cur_idx..]
    }

    fn track_field(&mut self, slot_off: VOffsetT, off: UOffsetT) {
        let fl = FieldLoc {
            id: slot_off,
            off: off,
        };
        self.field_locs.push(fl);
        self.max_voffset = max(self.max_voffset, slot_off);
    }
    pub fn start_table(&mut self) -> Offset<TableOffset> {
        self.assert_not_nested();
        self.nested = true;

        self.field_locs.clear();

        Offset::new(self.get_size() as UOffsetT)
    }
    fn grow_owned_buf(&mut self) {
        let starting_active_size = self.get_size();

        let old_len = self.owned_buf.len();
        let new_len = max(1, old_len * 2);

        assert!(
            new_len <= FLATBUFFERS_MAX_BUFFER_SIZE,
            "cannot grow buffer beyond 2 gigabytes"
        );

        let diff = new_len - old_len;
        self.owned_buf.resize(new_len, 0);
        self.cur_idx += diff;

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
            right.copy_from_slice(left);
        }
        // then, zero out the old end data (just to be safe).
        // this should be vectorized by rustc. rust has no stdlib memset.
        for x in &mut self.owned_buf[..middle] {
            *x = 0;
        }
    }
    fn assert_nested(&self, msg: &'static str) {
        assert!(self.nested, msg);
        // we don't assert that self.field_locs.len() >0 because the vtable
        // could be empty (e.g. for empty tables, or for all-default values).
    }
    fn assert_not_nested(&self) {
        assert!(!self.nested);
    }
    fn assert_finished(&self) {
        assert!(self.finished);
    }
    fn assert_not_finished(&self) {
        assert!(!self.finished);
    }
    pub fn start_vector(&mut self, len: usize, elem_size: usize) -> UOffsetT {
        self.assert_not_nested();
        self.nested = true;
        self.align(len * elem_size, SIZE_UOFFSET);
        self.align(len * elem_size, elem_size); // Just in case elemsize > uoffset_t.
        self.rev_cur_idx()
    }
    // Offset relative to the end of the buffer.
    pub fn rev_cur_idx(&self) -> UOffsetT {
        (self.owned_buf.len() - self.cur_idx) as UOffsetT
    }
    pub fn end_vector<T: 'fbb>(&mut self, num_elems: usize) -> Offset<Vector<'fbb, T>> {
        self.assert_nested("end_vector must be called after a call to start_vector");
        self.nested = false;
        let off = self.push::<UOffsetT>(num_elems as UOffsetT);
        Offset::new(off)
    }
    pub fn get_size(&self) -> usize {
        self.owned_buf.len() - self.cur_idx as usize
    }
    fn fill(&mut self, zero_pad_bytes: usize) {
        self.make_space(zero_pad_bytes);
    }
    // utf-8 string creation
    pub fn create_string(&mut self, s: &str) -> Offset<&'fbb str> {
        self.assert_not_nested();
        self.push(ZeroTerminatedByteSlice{0: s.as_bytes()});
        Offset::new(self.get_size() as UOffsetT)
        //Offset::<&str>::new(self.create_byte_string(s.as_bytes()).value())
    }
    pub fn create_byte_string(&mut self, data: &[u8]) -> Offset<&'fbb [u8]> {
        self.assert_not_nested();
        self.push(ZeroTerminatedByteSlice{0: data});
        Offset::new(self.get_size() as UOffsetT)
    }
    pub fn create_byte_vector<'a, 'b>(&'a mut self, data: &'b [u8]) -> Offset<Vector<'fbb, u8>> {
        self.assert_not_nested();
        self.push(data);
        Offset::new(self.get_size() as UOffsetT)
    }
    pub fn create_vector_of_strings<'a, 'b>(&'a mut self, xs: &'b [&'b str]) -> Offset<Vector<'fbb, ForwardsUOffset<&'fbb str>>> {
        let mut offsets: smallvec::SmallVec<[Offset<&str>; 0]> = smallvec::SmallVec::with_capacity(xs.len());
        unsafe { offsets.set_len(xs.len()); }
        for (i, &s) in xs.iter().enumerate().rev() {
            let o = self.create_string(s);
            offsets[i] = o;
        }
        self.create_vector(&offsets[..])
    }
    pub fn create_vector<'a, T: Push + Copy + 'fbb>(&'a mut self, items: &'a [T]) -> Offset<Vector<'fbb, T::Output>> {
        let elemsize = size_of::<T>();
        self.start_vector(elemsize, items.len());
        for i in (0..items.len()).rev() {
            self.push(items[i]);
        }
        Offset::new(self.end_vector::<T::Output>(items.len()).value())
    }
    pub fn end_table(&mut self, off: Offset<TableOffset>) -> Offset<TableOffset> {
        self.assert_nested("end_table must be called after a call to start_table");
        let n = self.write_vtable(off.value());
        self.nested = false;
        self.field_locs.clear();
        self.max_voffset = 0;
        let o = Offset::new(n);
        o
    }
    fn write_vtable(&mut self, table_tail_revloc: UOffsetT) -> UOffsetT {
        self.assert_nested("write_vtable must be called after a call to start_table");

        // Write the vtable offset, which is the start of any Table.
        // We fill its value later.
        //let object_vtable_revloc: UOffsetT = self.push_element_scalar::<SOffsetT>(0x99999999 as SOffsetT);
        let object_vtable_revloc: UOffsetT =
            self.push::<UOffsetT>(0xF0F0F0F0 as UOffsetT);
        //println!("just wrote filler: {:?}", self.get_active_buf_slice());

        // Layout of the data this function will create when a new vtable is
        // needed.
        // --------------------------------------------------------------------
        // vtable starts here
        // | x, x -- vtable len (bytes) [u16]
        // | x, x -- object inline len (bytes) [u16]
        // | x, x -- zero, or num bytes from start of object to field #0   [u16]
        // | ...
        // | x, x -- zero, or num bytes from start of object to field #n-1 [u16]
        // vtable ends here
        // table starts here
        // | x, x, x, x -- offset (negative direction) to our vtable [i32]
        // |               aka "vtableoffset"
        // | -- table inline data begins here, we don't touch it --
        // table ends here -- aka "table_start"
        // --------------------------------------------------------------------
        //
        // Layout of the data this function will create when we re-use an
        // existing vtable.
        //
        // We always serialize this particular vtable, then compare it to the
        // other vtables we know about to see if there is a duplicate. If there
        // is, then we erase the serialized vtable we just made.
        // We serialize it first so that we are able to do byte-by-byte
        // comparisons with already-serialized vtables. This 1) saves
        // bookkeeping space (we only keep revlocs to existing vtables), 2)
        // allows us to convert to little-endian once, then do
        // fast memcmp comparisons, and 3) by ensuring we are comparing real
        // serialized vtables, we can be more assured that we are doing the
        // comparisons correctly.
        //
        // --------------------------------------------------------------------
        // table starts here
        // | x, x, x, x -- offset (negative direction) to an existing vtable [i32]
        // |               aka "vtableoffset"
        // | -- table inline data begins here, we don't touch it --
        // table starts here: aka "table_start"
        // --------------------------------------------------------------------

        // Write a vtable, which consists entirely of voffset_t elements.
        // It starts with the number of offsets, followed by a type id, followed
        // by the offsets themselves. In reverse:
        // Include space for the last offset and ensure empty tables have a
        // minimum size.
        let vtable_len = max(self.max_voffset + SIZE_VOFFSET as VOffsetT,
                             field_index_to_field_offset(0)) as usize;
        self.fill(vtable_len);
        let table_object_size = object_vtable_revloc - table_tail_revloc;
        debug_assert!(table_object_size < 0x10000); // Vtable use 16bit offsets.

        let vt_start_pos = self.cur_idx;
        let vt_end_pos = self.cur_idx + vtable_len;
        {
            let vtfw = &mut VTableWriter::init(&mut self.owned_buf[vt_start_pos..vt_end_pos]);
            vtfw.write_vtable_byte_length(vtable_len as VOffsetT);
            vtfw.write_object_inline_size(table_object_size as VOffsetT);
            for &fl in self.field_locs.iter() {
                let pos: VOffsetT = (object_vtable_revloc - fl.off) as VOffsetT;
                debug_assert_eq!(vtfw.get_field_offset(fl.id),
                                 0,
                                 "tried to write a vtable field multiple times");
                vtfw.write_field_offset(fl.id, pos);
            }
        }
        let vt_use = {
            let mut ret: usize = self.get_size();

            // LIFO order
            for &vt_rev_pos in self.written_vtable_revpos.iter().rev() {
                let eq = {
                    let this_vt = VTable::init(&self.owned_buf[..], self.cur_idx);
                    let other_vt = VTable::init(&self.owned_buf[..], self.cur_idx + self.get_size() - vt_rev_pos as usize);
                    other_vt == this_vt
                };
                if eq {
                    VTableWriter::init(&mut self.owned_buf[vt_start_pos..vt_end_pos]).clear();
                    self.cur_idx += vtable_len;
                    ret = vt_rev_pos as usize;
                    break;
                }
            }
            ret
        };
        if vt_use == self.get_size() {
            let n = self.rev_cur_idx();
            self.written_vtable_revpos.push(n);
        }

        {
            let n = self.cur_idx + self.get_size() - object_vtable_revloc as usize;
            let saw = read_scalar::<UOffsetT>(&self.owned_buf[n..n + SIZE_SOFFSET]);
            debug_assert_eq!(saw, 0xF0F0F0F0);
            emplace_scalar::<SOffsetT>(
                &mut self.owned_buf[n..n + SIZE_SOFFSET],
                vt_use as SOffsetT - object_vtable_revloc as SOffsetT,
            );
        }

        self.field_locs.clear();
        self.max_voffset = 0;

        object_vtable_revloc
    }
    pub fn finish_size_prefixed<T>(&mut self, root: Offset<T>, file_identifier: Option<&str>) {
        self.finish_with_opts(root, file_identifier, true);
    }
    pub fn finish<T>(&mut self, root: Offset<T>, file_identifier: Option<&str>) {
        self.finish_with_opts(root, file_identifier, false);
    }
    pub fn finish_minimal<T>(&mut self, root: Offset<T>) {
        self.finish_with_opts(root, None, false);
    }
    // with or without a size prefix changes how we load the data, so finish*
    // functions are split along those lines.
    fn finish_with_opts<T>(
        &mut self,
        root: Offset<T>,
        file_identifier: Option<&str>,
        size_prefixed: bool,
    ) {
        self.assert_not_finished();
        self.assert_not_nested();
        self.written_vtable_revpos.clear();

        let to_align = {
            // for the root offset:
            let a = SIZE_UOFFSET;
            // for the size prefix:
            let b = if size_prefixed { SIZE_UOFFSET } else { 0 };
            // for the file identifier (a string that is not zero-terminated):
            let c = if file_identifier.is_some() {
                FILE_IDENTIFIER_LENGTH
            } else {
                0
            };
            a + b + c
        };

        {
            let ma = self.min_align;
            self.align(to_align, ma);
        }

        if let Some(ident) = file_identifier {
            assert_eq!(ident.len(), FILE_IDENTIFIER_LENGTH);
            self.push_bytes(ident.as_bytes());
        }

        {
            self.push(root);
        }

        if size_prefixed {
            let sz = self.get_size() as UOffsetT;
            self.push::<UOffsetT>(sz);
        }
        self.finished = true;
    }

    fn align(&mut self, len: usize, alignment: usize) {
        self.track_min_align(alignment);
        let s = self.get_size() as usize;
        self.fill(padding_bytes(s + len, alignment));
    }
    fn track_min_align(&mut self, alignment: usize) {
        self.min_align = max(self.min_align, alignment);
    }
    pub fn push<X: Push>(&mut self, x: X) -> UOffsetT {
        let ap = x.align_params();

        self.align(ap.len, ap.alignment);
        self.make_space(ap.len);
        {
            let (dst, rest) = (&mut self.owned_buf[self.cur_idx..]).split_at_mut(ap.len);
            x.do_write(dst, rest);
        }
        self.get_size() as UOffsetT
    }
    pub fn push_slot<X: Push + PartialEq>(&mut self, slotoff: VOffsetT, x: X, d: Option<X>) {
        self.assert_nested("push_slot must be called after start_table");
        if d.is_some() && x == d.unwrap() {
            return;
        }
        self.push_slot_always(slotoff, x);
    }
    pub fn push_slot_always<X: Push>(&mut self, slotoff: VOffsetT, x: X) {
        self.assert_nested("push_slot_always must be called after start_table");
        let off = self.push(x);
        self.track_field(slotoff, off);
    }
    pub fn push_bytes(&mut self, x: &[u8]) -> UOffsetT {
        let n = self.make_space(x.len());
        &mut self.owned_buf[n..n + x.len()].copy_from_slice(x);

        n as UOffsetT
    }
    pub fn make_space(&mut self, want: usize) -> usize {
        self.ensure_capacity(want);
        self.cur_idx -= want;
        self.cur_idx
    }
    pub fn ensure_capacity(&mut self, want: usize) -> usize {
        if self.unused_ready_space() >= want {
            return want;
        }
        assert!(
            want <= FLATBUFFERS_MAX_BUFFER_SIZE,
            "cannot grow buffer beyond 2 gigabytes"
        );
        while self.unused_ready_space() < want {
            self.grow_owned_buf();
        }
        want
    }
    fn unused_ready_space(&self) -> usize {
        self.cur_idx
    }
    pub fn finished_bytes(&self) -> &[u8] {
        self.assert_finished();
        &self.owned_buf[self.cur_idx..]
    }
    pub fn required(
        &self,
        tab_revloc: Offset<TableOffset>,
        slot_byte_loc: VOffsetT,
        assert_msg_name: &'static str,
    ) {
        let tab = Table::new(
            &self.owned_buf[..],
            self.cur_idx + (self.get_size() - tab_revloc.value() as usize),
        );
        let o = tab.vtable().get(slot_byte_loc) as usize;
        assert!(o != 0, "missing required field {}", assert_msg_name);
    }
}

pub fn to_bytes<'a, T: 'a + Sized>(t: &'a T) -> &'a [u8] {
    let sz = size_of::<T>();
    unsafe { from_raw_parts((t as *const T) as *const u8, sz) }
}

#[inline]
pub fn padding_bytes(buf_size: usize, scalar_size: usize) -> usize {
    // ((!buf_size) + 1) & (scalar_size - 1)
    (!buf_size).wrapping_add(1) & (scalar_size.wrapping_sub(1))
}
