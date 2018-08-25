
pub trait GeneratedStruct {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Table<'a> {
    pub buf: &'a [u8],
    pub loc: usize,
}

impl<'a> Table<'a> {
    pub fn new(buf: &'a [u8], loc: usize) -> Self {
        Table { buf: buf, loc: loc }
    }
    #[inline]
    pub fn vtable(&'a self) -> VTable<'a> {
        <BackwardsSOffset<VTable<'a>>>::follow(self.buf, self.loc)
    }
    pub fn get<T: Follow<'a> + 'a>(
        &'a self,
        slot_byte_loc: VOffsetT,
        default: Option<T::Inner>,
    ) -> Option<T::Inner> {
        let o = self.vtable().get(slot_byte_loc) as usize;
        if o == 0 {
            return default;
        }
        Some(<T>::follow(self.buf, self.loc + o))
    }
}
