use follow::Follow;
use primitives::*;
use vtable::VTable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Table<'a> {
    pub buf: &'a [u8],
    pub loc: usize,
}

impl<'a> Table<'a> {
    #[inline]
    pub fn new(buf: &'a [u8], loc: usize) -> Self {
        Table { buf: buf, loc: loc }
    }
    #[inline]
    pub fn vtable(&'a self) -> VTable<'a> {
        <BackwardsSOffset<VTable<'a>>>::follow(self.buf, self.loc)
    }
    #[inline]
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

impl<'a> Follow<'a> for Table<'a> {
    type Inner = Table<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Table { buf: buf, loc: loc }
    }
}

#[inline]
pub fn get_root<'a, T: Follow<'a> + 'a>(data: &'a [u8]) -> T::Inner {
    <ForwardsUOffset<T>>::follow(data, 0)
}
#[inline]
pub fn get_size_prefixed_root<'a, T: Follow<'a> + 'a>(data: &'a [u8]) -> T::Inner {
    <SkipSizePrefix<ForwardsUOffset<T>>>::follow(data, 0)
}
#[inline]
pub fn buffer_has_identifier(data: &[u8], ident: &str, size_prefixed: bool) -> bool {
    assert_eq!(ident.len(), FILE_IDENTIFIER_LENGTH);

    let got = if size_prefixed {
        <SkipSizePrefix<SkipRootOffset<FileIdentifier>>>::follow(data, 0)
    } else {
        <SkipRootOffset<FileIdentifier>>::follow(data, 0)
    };

    ident.as_bytes() == got
}
