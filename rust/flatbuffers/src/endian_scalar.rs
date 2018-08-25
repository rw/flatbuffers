use std::mem::size_of;

pub trait EndianScalar: Sized + PartialEq + Copy + Clone {
    fn to_little_endian(self) -> Self;
    fn from_little_endian(self) -> Self;
}

impl EndianScalar for bool {
    fn to_little_endian(self) -> Self {
        self
    }
    fn from_little_endian(self) -> Self {
        self
    }
}
impl EndianScalar for u8 {
    fn to_little_endian(self) -> Self {
        self
    }
    fn from_little_endian(self) -> Self {
        self
    }
}
impl EndianScalar for i8 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for u16 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for i16 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for u32 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for i32 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for u64 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for i64 {
    fn to_little_endian(self) -> Self {
        Self::to_le(self)
    }
    fn from_little_endian(self) -> Self {
        Self::from_le(self)
    }
}
impl EndianScalar for f32 {
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
pub fn byte_swap_f32(x: f32) -> f32 {
    let mut ret = x;

    let ptr = &mut ret as *mut f32 as *mut u32;
    unsafe { *ptr }.swap_bytes();

    ret
}

#[allow(dead_code)]
pub fn byte_swap_f64(x: f64) -> f64 {
    let mut ret = x;

    let ptr = &mut ret as *mut f64 as *mut u64;
    unsafe { *ptr }.swap_bytes();

    ret
}

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
