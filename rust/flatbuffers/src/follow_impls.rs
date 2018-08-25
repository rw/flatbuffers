use follow::Follow;

impl<'a, T: Follow<'a>> Follow<'a> for ForwardsVOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_VOFFSET];
        let off = read_scalar::<u16>(slice) as usize;
        T::follow(buf, loc + off)
    }
}

impl<'a, T: Follow<'a>> Follow<'a> for BackwardsSOffset<T> {
    type Inner = T::Inner;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let slice = &buf[loc..loc + SIZE_SOFFSET];
        let off = read_scalar::<SOffsetT>(slice);
        T::follow(buf, (loc as SOffsetT - off) as usize)
    }
}

impl<'a> Follow<'a> for &'a str {
    type Inner = &'a str;
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let slice = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len];
        let s = unsafe { std::str::from_utf8_unchecked(slice) };
        s
    }
}

