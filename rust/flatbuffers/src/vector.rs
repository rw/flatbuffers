#[derive(Debug)]
pub struct Vector<'a, T: 'a>(&'a [u8], usize, PhantomData<T>);
