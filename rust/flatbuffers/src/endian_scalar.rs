use std::mem::size_of;

pub trait EndianScalar: Sized + PartialEq + Copy + Clone {
    fn to_little_endian(self) -> Self;
    fn from_little_endian(self) -> Self;
}

macro_rules! impl_endian_scalar_self {
    ($ty:ident) => (
        impl EndianScalar for $ty {
            #[inline(always)]
            fn to_little_endian(self) -> Self {
                self
            }
            #[inline(always)]
            fn from_little_endian(self) -> Self {
                self
            }
        }
    )
}

macro_rules! impl_endian_scalar_stdlib_le_conversion {
    ($ty:ident) => (
        impl EndianScalar for $ty {
            #[inline(always)]
            fn to_little_endian(self) -> Self {
                Self::to_le(self)
            }
            #[inline(always)]
            fn from_little_endian(self) -> Self {
                Self::from_le(self)
            }
        }
    )
}

impl_endian_scalar_self!(bool);
impl_endian_scalar_self!(u8);
impl_endian_scalar_self!(i8);

impl_endian_scalar_stdlib_le_conversion!(u16);
impl_endian_scalar_stdlib_le_conversion!(u32);
impl_endian_scalar_stdlib_le_conversion!(u64);
impl_endian_scalar_stdlib_le_conversion!(i16);
impl_endian_scalar_stdlib_le_conversion!(i32);
impl_endian_scalar_stdlib_le_conversion!(i64);


impl EndianScalar for f32 {
    #[inline(always)]
    fn to_little_endian(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            byte_swap_f32(&self)
        }
    }
    #[inline(always)]
    fn from_little_endian(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            byte_swap_f32(&self)
        }
    }
}
impl EndianScalar for f64 {
    #[inline(always)]
    fn to_little_endian(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            byte_swap_f64(&self)
        }
    }
    #[inline(always)]
    fn from_little_endian(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            byte_swap_f64(&self)
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn byte_swap_f32(x: f32) -> f32 {
    let mut ret = x;

    let ptr = &mut ret as *mut f32 as *mut u32;
    unsafe { *ptr }.swap_bytes();

    ret
}

#[allow(dead_code)]
#[inline(always)]
pub fn byte_swap_f64(x: f64) -> f64 {
    let mut ret = x;

    let ptr = &mut ret as *mut f64 as *mut u64;
    unsafe { *ptr }.swap_bytes();

    ret
}

#[inline(always)]
pub fn emplace_scalar<T: EndianScalar>(s: &mut [u8], x: T) {
    let sz = size_of::<T>();
    debug_assert!(s.len() >= sz);

    let mut_ptr = s.as_mut_ptr() as *mut T;
    let val = x.to_little_endian();
    unsafe {
        *mut_ptr = val;
    }
}
pub fn read_scalar_at<T: EndianScalar>(s: &[u8], loc: usize) -> T {
    let buf = &s[loc..loc + size_of::<T>()];
    read_scalar(buf)
}
pub fn read_scalar<T: EndianScalar>(s: &[u8]) -> T {
    let sz = size_of::<T>();
    debug_assert!(s.len() >= sz);

    let p = s.as_ptr() as *const T;
    let x = unsafe { *p };

    x.from_little_endian()
}

