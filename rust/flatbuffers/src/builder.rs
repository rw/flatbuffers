extern crate smallvec;

use std::cmp::max;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::write_bytes;

use endian_scalar::{read_scalar, emplace_scalar};
use primitives::*;
use push::{Push, ZeroTerminatedByteSlice};
use table::Table;
use vtable::{VTable, field_index_to_field_offset};
use vtable_writer::VTableWriter;
use vector::Vector;

#[derive(Clone, Copy, Debug)]
struct FieldLoc {
    off: UOffsetT,
    id: VOffsetT,
}

/// FlatBufferBuilder builds a FlatBuffer through manipulating its internal
/// state. It has an owned `Vec<u8>` that grows as needed (up to the hardcoded
/// limit of 2GiB, which is set by the FlatBuffers format).
///
/// Users are expected to create/reset instances of this type; however, its
/// functions are primarily intended for use by the generated code emitted by
/// the `flatc` compiler.
pub struct FlatBufferBuilder<'fbb> {
    owned_buf: Vec<u8>,
    head: usize,

    field_locs: Vec<FieldLoc>,
    written_vtable_revpos: Vec<UOffsetT>,

    nested: bool,
    finished: bool,

    min_align: usize,

    _phantom: PhantomData<&'fbb ()>,
}
impl<'fbb> FlatBufferBuilder<'fbb> {
    /// Create a FlatBufferBuilder that is ready for writing.
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    /// Create a FlatBufferBuilder that is ready for writing, with a
    /// ready-to-use capacity of the provided size.
    ///
    /// To minimize memory allocations after initialization, call this function
    /// with the constant `FLATBUFFERS_MAX_BUFFER_SIZE`.
    pub fn new_with_capacity(size: usize) -> Self {
        assert!(size <= FLATBUFFERS_MAX_BUFFER_SIZE,
                "cannot initialize buffer bigger than 2 gigabytes");
        FlatBufferBuilder {
            owned_buf: vec![0u8; size],
            head: size,

            field_locs: Vec::new(),
            written_vtable_revpos: Vec::new(),

            nested: false,
            finished: false,

            min_align: 0,

            _phantom: PhantomData,
        }
    }

    /// Reset the FlatBufferBuilder in order to reduce heap allocations.
    /// If you are using a FlatBufferBuilder repeatedly, make sure to use this
    /// function!
    pub fn reset(&mut self) {
        // memset only the part of the buffer that could be dirty:
        {
            let to_clear = self.owned_buf.len() - self.head;
            let ptr = (&mut self.owned_buf[self.head..]).as_mut_ptr();
            unsafe { write_bytes(ptr, 0, to_clear); }
        }

        self.head = self.owned_buf.len();
        self.written_vtable_revpos.clear();

        self.nested = false;
        self.finished = false;

        self.min_align = 0;
    }

    /// Destroy the FlatBufferBuilder, returning its internal byte vector
    /// and the index into it that represents the start of valid data.
    pub fn collapse(self) -> (Vec<u8>, usize) {
        (self.owned_buf, self.head)
    }

    #[inline]
    pub fn push<X: Push>(&mut self, x: X) -> WIPOffset<X::Output> {
        self.align(x.size(), x.alignment());
        self.make_space(x.size());
        {
            let (dst, rest) = (&mut self.owned_buf[self.head..]).split_at_mut(x.size());
            x.push(dst, rest);
        }
        WIPOffset::new(self.used_space() as UOffsetT)
    }
    pub fn push_slot<X: Push + PartialEq>(&mut self, slotoff: VOffsetT, x: X, d: X) {
        self.assert_nested("push_slot must be called after start_table");
        if x == d {
            return;
        }
        self.push_slot_always(slotoff, x);
    }
    pub fn push_slot_always<X: Push>(&mut self, slotoff: VOffsetT, x: X) {
        self.assert_nested("push_slot_always must be called after start_table");
        let off = self.push(x);
        self.track_field(slotoff, off.value());
    }

    pub fn num_written_vtables(&self) -> usize {
        self.written_vtable_revpos.len()
    }
    pub fn get_active_buf_slice<'a>(&'a self) -> &'a [u8] {
        &self.owned_buf[self.head..]
    }

    fn track_field(&mut self, slot_off: VOffsetT, off: UOffsetT) {
        let fl = FieldLoc {
            id: slot_off,
            off: off,
        };
        self.field_locs.push(fl);
    }
    pub fn start_table(&mut self) -> WIPOffset<TableUnfinishedWIPOffset> {
        self.assert_not_nested("start_table can not be called when a table or vector is under construction");
        self.nested = true;

        WIPOffset::new(self.used_space() as UOffsetT)
    }
    pub fn end_table(&mut self, off: WIPOffset<TableUnfinishedWIPOffset>) -> WIPOffset<TableFinishedWIPOffset> {
        self.assert_nested("end_table must be called after a call to start_table");

        let o = self.write_vtable(off);

        self.nested = false;
        self.field_locs.clear();

        WIPOffset::new(o.value())
    }
    pub fn start_vector(&mut self, len: usize, elem_size: usize) {
        self.assert_not_nested("start_vector can not be called when a table or vector is under construction");
        self.nested = true;
        self.align(len * elem_size, SIZE_UOFFSET);
        self.align(len * elem_size, elem_size); // Just in case elemsize > uoffset_t.
    }
    pub fn end_vector<T: 'fbb>(&mut self, num_elems: usize) -> WIPOffset<Vector<'fbb, T>> {
        self.assert_nested("end_vector must be called after a call to start_vector");
        self.nested = false;
        let o = self.push::<UOffsetT>(num_elems as UOffsetT);
        WIPOffset::new(o.value())
    }
    #[inline]
    pub fn used_space(&self) -> usize {
        self.owned_buf.len() - self.head as usize
    }
    #[inline]
    fn fill(&mut self, zero_pad_bytes: usize) {
        self.make_space(zero_pad_bytes);
    }
    pub fn create_string(&mut self, s: &str) -> WIPOffset<&'fbb str> {
        self.assert_not_nested("create_string can not be called when a table or vector is under construction");
        self.push(ZeroTerminatedByteSlice::new(s.as_bytes()));
        WIPOffset::new(self.used_space() as UOffsetT)
    }
    pub fn create_byte_string(&mut self, data: &[u8]) -> WIPOffset<&'fbb [u8]> {
        self.assert_not_nested("create_byte_string can not be called when a table or vector is under construction");
        self.push(ZeroTerminatedByteSlice::new(data));
        WIPOffset::new(self.used_space() as UOffsetT)
    }
    pub fn create_vector_of_strings<'a, 'b>(&'a mut self, xs: &'b [&'b str]) -> WIPOffset<Vector<'fbb, ForwardsUOffset<&'fbb str>>> {
        let mut offsets: smallvec::SmallVec<[WIPOffset<&str>; 0]> = smallvec::SmallVec::with_capacity(xs.len());
        unsafe { offsets.set_len(xs.len()); }
        for (i, &s) in xs.iter().enumerate().rev() {
            let o = self.create_string(s);
            offsets[i] = o;
        }
        self.create_vector(&offsets[..])
    }
    pub fn create_vector<'a, T: Push + Copy + 'fbb>(&'a mut self, items: &'a [T]) -> WIPOffset<Vector<'fbb, T::Output>> {
        let elemsize = size_of::<T>();
        self.start_vector(elemsize, items.len());
        for i in (0..items.len()).rev() {
            self.push(items[i]);
        }
        WIPOffset::new(self.end_vector::<T::Output>(items.len()).value())
    }
    fn write_vtable(&mut self, table_tail_revloc: WIPOffset<TableUnfinishedWIPOffset>) -> WIPOffset<VTableWIPOffset> {
        self.assert_nested("write_vtable must be called after a call to start_table");

        // Write the vtable offset, which is the start of any Table.
        // We fill its value later.
        let object_vtable_revloc: WIPOffset<VTableWIPOffset> =
            WIPOffset::new(self.push::<UOffsetT>(0xF0F0F0F0 as UOffsetT).value());

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

        // Include space for the last offset and ensure empty tables have a
        // minimum size.
        let max_voffset = self.field_locs.iter().map(|fl| fl.id).max();
        let vtable_len = match max_voffset {
            None => { field_index_to_field_offset(0) as usize }
            Some(mv) => { mv as usize + SIZE_VOFFSET }
        };
        self.fill(vtable_len);
        let table_object_size = object_vtable_revloc.value() - table_tail_revloc.value();
        debug_assert!(table_object_size < 0x10000); // Vtable use 16bit offsets.

        let vt_start_pos = self.head;
        let vt_end_pos = self.head + vtable_len;
        {
            let vtfw = &mut VTableWriter::init(&mut self.owned_buf[vt_start_pos..vt_end_pos]);
            vtfw.write_vtable_byte_length(vtable_len as VOffsetT);
            vtfw.write_object_inline_size(table_object_size as VOffsetT);
            for &fl in self.field_locs.iter() {
                let pos: VOffsetT = (object_vtable_revloc.value() - fl.off) as VOffsetT;
                debug_assert_eq!(vtfw.get_field_offset(fl.id),
                                 0,
                                 "tried to write a vtable field multiple times");
                vtfw.write_field_offset(fl.id, pos);
            }
        }
        let vt_use = {
            let mut ret: usize = self.used_space();

            // LIFO order
            for &vt_rev_pos in self.written_vtable_revpos.iter().rev() {
                let eq = {
                    let this_vt = VTable::init(&self.owned_buf[..], self.head);
                    let other_vt = VTable::init(&self.owned_buf[..], self.head + self.used_space() - vt_rev_pos as usize);
                    other_vt == this_vt
                };
                if eq {
                    VTableWriter::init(&mut self.owned_buf[vt_start_pos..vt_end_pos]).clear();
                    self.head += vtable_len;
                    ret = vt_rev_pos as usize;
                    break;
                }
            }
            ret
        };

        if vt_use == self.used_space() {
            self.written_vtable_revpos.push(vt_use as UOffsetT);
        }

        {
            let n = self.head + self.used_space() - object_vtable_revloc.value() as usize;
            let saw = read_scalar::<UOffsetT>(&self.owned_buf[n..n + SIZE_SOFFSET]);
            debug_assert_eq!(saw, 0xF0F0F0F0);
            emplace_scalar::<SOffsetT>(
                &mut self.owned_buf[n..n + SIZE_SOFFSET],
                vt_use as SOffsetT - object_vtable_revloc.value() as SOffsetT,
            );
        }

        self.field_locs.clear();

        object_vtable_revloc
    }
    fn grow_owned_buf(&mut self) {
        let old_len = self.owned_buf.len();
        let new_len = max(1, old_len * 2);

        assert!(new_len <= FLATBUFFERS_MAX_BUFFER_SIZE,
                "cannot grow buffer beyond 2 gigabytes");

        let starting_active_size = self.used_space();

        let diff = new_len - old_len;
        self.owned_buf.resize(new_len, 0);
        self.head += diff;

        let ending_active_size = self.used_space();
        debug_assert_eq!(starting_active_size, ending_active_size);

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
        // finally, zero out the old end data.
        {
            let ptr = (&mut self.owned_buf[..middle]).as_mut_ptr();
            unsafe { write_bytes(ptr, 0, middle); }
        }
    }
    #[inline]
    pub fn finish_size_prefixed<T>(&mut self, root: WIPOffset<T>, file_identifier: Option<&str>) {
        self.finish_with_opts(root, file_identifier, true);
    }
    #[inline]
    pub fn finish<T>(&mut self, root: WIPOffset<T>, file_identifier: Option<&str>) {
        self.finish_with_opts(root, file_identifier, false);
    }
    #[inline]
    pub fn finish_minimal<T>(&mut self, root: WIPOffset<T>) {
        self.finish_with_opts(root, None, false);
    }
    // with or without a size prefix changes how we load the data, so finish*
    // functions are split along those lines.
    fn finish_with_opts<T>(&mut self,
                           root: WIPOffset<T>,
                           file_identifier: Option<&str>,
                           size_prefixed: bool) {
        self.assert_not_finished("buffer cannot be finished when it is already finished");
        self.assert_not_nested("buffer cannot be finished when a table or vector is under construction");
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
            debug_assert_eq!(ident.len(), FILE_IDENTIFIER_LENGTH);
            self.push_bytes_unprefixed(ident.as_bytes());
        }

        self.push(root);

        if size_prefixed {
            let sz = self.used_space() as UOffsetT;
            self.push::<UOffsetT>(sz);
        }
        self.finished = true;
    }

    fn align(&mut self, len: usize, alignment: usize) {
        self.track_min_align(alignment);
        let s = self.used_space() as usize;
        self.fill(padding_bytes(s + len, alignment));
    }
    fn track_min_align(&mut self, alignment: usize) {
        self.min_align = max(self.min_align, alignment);
    }
    fn push_bytes_unprefixed(&mut self, x: &[u8]) -> UOffsetT {
        let n = self.make_space(x.len());
        &mut self.owned_buf[n..n + x.len()].copy_from_slice(x);

        n as UOffsetT
    }
    fn make_space(&mut self, want: usize) -> usize {
        self.ensure_capacity(want);
        self.head -= want;
        self.head
    }
    fn ensure_capacity(&mut self, want: usize) -> usize {
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
    #[inline]
    fn unused_ready_space(&self) -> usize {
        self.head
    }
    pub fn unfinished_data(&self) -> &[u8] {
        &self.owned_buf[self.head..]
    }
    pub fn finished_data(&self) -> &[u8] {
        self.assert_finished("finished_bytes cannot be called when the buffer is not yet finished");
        &self.owned_buf[self.head..]
    }
    pub fn required(&self,
                    tab_revloc: WIPOffset<TableFinishedWIPOffset>,
                    slot_byte_loc: VOffsetT,
                    assert_msg_name: &'static str) {
        let idx = self.used_space() - tab_revloc.value() as usize;
        let tab = Table::new(&self.owned_buf[self.head..], idx);
        let o = tab.vtable().get(slot_byte_loc) as usize;
        assert!(o != 0, "missing required field {}", assert_msg_name);
    }
    fn assert_nested(&self, msg: &'static str) {
        assert!(self.nested, msg);
        // we don't assert that self.field_locs.len() >0 because the vtable
        // could be empty (e.g. for empty tables, or for all-default values).
    }
    fn assert_not_nested(&self, msg: &'static str) {
        assert!(!self.nested, msg);
    }
    fn assert_finished(&self, msg: &'static str) {
        assert!(self.finished, msg);
    }
    fn assert_not_finished(&self, msg: &'static str) {
        assert!(!self.finished, msg);
    }
}

#[inline]
pub fn padding_bytes(buf_size: usize, scalar_size: usize) -> usize {
    // ((!buf_size) + 1) & (scalar_size - 1)
    (!buf_size).wrapping_add(1) & (scalar_size.wrapping_sub(1))
}
