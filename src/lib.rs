use std::{mem, ptr, marker::PhantomData};

#[cfg(feature = "extras")]
mod euclid;

// Due to a lack of specialization, `Pod` is used to opt into `memcpy` behavior
// because a blanket impl for `Copy` doesn't allow specialization for `()`,
// `Option<T: Copy>`, etc.
pub unsafe trait UnsafePod: Copy {}

unsafe impl UnsafePod for bool {}

unsafe impl UnsafePod for i8 {}
unsafe impl UnsafePod for i16 {}
unsafe impl UnsafePod for i32 {}
unsafe impl UnsafePod for i64 {}
unsafe impl UnsafePod for isize {}

unsafe impl UnsafePod for u8 {}
unsafe impl UnsafePod for u16 {}
unsafe impl UnsafePod for u32 {}
unsafe impl UnsafePod for u64 {}
unsafe impl UnsafePod for usize {}

unsafe impl UnsafePod for f32 {}
unsafe impl UnsafePod for f64 {}

unsafe impl UnsafePod for () {}

pub unsafe trait UnsafePokable {
    fn poke_pod<T: UnsafePod>(self, v: T) -> Self;
    fn poke_bytes(self, v: &[u8]) -> Self;
}

unsafe impl UnsafePokable for *mut u8 {
    fn poke_pod<T: UnsafePod>(self, v: T) -> Self {
        unsafe {
            ptr::copy_nonoverlapping(&v as *const T as *const u8, self, mem::size_of::<T>());
            self.add(mem::size_of::<T>())
        }
    }

    fn poke_bytes(self, v: &[u8]) -> Self {
        unsafe {
            ptr::copy_nonoverlapping(v.as_ptr(), self, v.len());
            self.add(v.len())
        }
    }
}

pub trait UnsafePoke {
    unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP;
    fn poke_max_size() -> usize;
}

impl<T: UnsafePod> UnsafePoke for T {
    unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        up.poke_pod(*self)
    }

    fn poke_max_size() -> usize {
        mem::size_of::<T>()
    }
}

impl<T> UnsafePoke for Option<T>
where
    T: UnsafePoke,
{
    unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        if let Some(ref value) = self {
            let up = 1u8.poke(up);
            value.poke(up)
        } else {
            0u8.poke(up)
        }
    }

    fn poke_max_size() -> usize {
        1 + T::poke_max_size()
    }
}

// Does not require T: UnsafePoke.
impl<T> UnsafePoke for [T; 0] {
    unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        up
    }

    fn poke_max_size() -> usize {
        0
    }
}

macro_rules! impl_unsafe_poke_arrays {
    ($($len:tt)+) => {
        $(impl<T: UnsafePoke> UnsafePoke for [T; $len] {
            unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
                self.iter().fold(up, |up, e| e.poke(up))
            }

            fn poke_max_size() -> usize {
                T::poke_max_size() * $len
            }
        })+
    }
}

impl_unsafe_poke_arrays! {
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

macro_rules! impl_unsafe_poke_tuples {
    ($(($($n:tt $name:ident)+))+) => {
        $(impl<$($name: UnsafePoke),+> UnsafePoke for ($($name,)+) {
            unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
                $(let up = self.$n.poke(up);
                )+
                up
            }

            fn poke_max_size() -> usize {
                0 $( + $name::poke_max_size())+
            }
        })+
    }
}

impl_unsafe_poke_tuples! {
    (0 T0)
    (0 T0 1 T1)
    (0 T0 1 T1 2 T2)
    (0 T0 1 T1 2 T2 3 T3)
    (0 T0 1 T1 2 T2 3 T3 4 T4)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<T> UnsafePoke for PhantomData<T> {
    unsafe fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        up
    }

    fn poke_max_size() -> usize {
        0
    }
}
