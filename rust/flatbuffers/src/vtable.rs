
#[derive(Debug)]
pub struct VTable<'a> {
    buf: &'a [u8],
    loc: usize,
}


impl<'a> PartialEq for VTable<'a> {
    fn eq(&self, other: &VTable) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl<'a> VTable<'a> {
    pub fn num_fields(&self) -> usize {
        (self.num_bytes() / SIZE_VOFFSET) - 2
    }
    pub fn num_bytes(&self) -> usize {
        read_scalar_at::<VOffsetT>(self.buf, self.loc) as usize
    }
    pub fn object_inline_num_bytes(&self) -> usize {
        let n = read_scalar_at::<VOffsetT>(self.buf, self.loc + SIZE_VOFFSET);
        n as usize
    }
    pub fn get_field(&self, idx: usize) -> VOffsetT {
        // TODO(rw): distinguish between None and 0?
        if idx > self.num_fields() {
            return 0;
        }
        read_scalar_at::<VOffsetT>(
            self.buf,
            self.loc + SIZE_VOFFSET + SIZE_VOFFSET + SIZE_VOFFSET * idx,
        )
    }
    pub fn get(&self, byte_loc: VOffsetT) -> VOffsetT {
        // TODO(rw): distinguish between None and 0?
        if byte_loc as usize >= self.num_bytes() {
            return 0;
        }
        read_scalar_at::<VOffsetT>(self.buf, self.loc + byte_loc as usize)
    }
    pub fn as_bytes(&self) -> &[u8] {
        let len = self.num_bytes();
        &self.buf[self.loc..self.loc + len]
    }
}


pub fn field_index_to_field_offset(field_id: VOffsetT) -> VOffsetT {
    // Should correspond to what end_table() below builds up.
    let fixed_fields = 2; // Vtable size and Object Size.
    ((field_id + fixed_fields) * (SIZE_VOFFSET as VOffsetT)) as VOffsetT
}
pub fn field_offset_to_field_index(field_o: VOffsetT) -> VOffsetT {
    debug_assert!(field_o >= 2);
    let fixed_fields = 2; // VTable size and Object Size.
    (field_o / (SIZE_VOFFSET as VOffsetT)) - fixed_fields
}
