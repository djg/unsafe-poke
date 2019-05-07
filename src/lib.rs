use std::{mem, ptr, marker::PhantomData};

pub unsafe trait UnsafePokable {
    fn poke_i8(self, v: i8) -> Self;
    fn poke_i16(self, v: i16) -> Self;
    fn poke_i32(self, v: i32) -> Self;
    fn poke_i64(self, v: i64) -> Self;
    fn poke_u8(self, v: u8) -> Self;
    fn poke_u16(self, v: u16) -> Self;
    fn poke_u32(self, v: u32) -> Self;
    fn poke_u64(self, v: u64) -> Self;
    fn poke_f32(self, v: f32) -> Self;
    fn poke_f64(self, v: f64) -> Self;
    fn poke_bytes(self, v: &[u8]) -> Self;
}

macro_rules! impl_ptr_write {
    ($($mem:ident: $ty:ty),*) => {
        $(fn $mem(self, v: $ty) -> Self {
            unsafe {
                ptr::write(self as *mut _, v);
                self.add(mem::size_of::<$ty>())
            }
        })*
    };
    ($($mem:ident: $ty:ty,)*) => {
        impl_ptr_write!{$($mem: $ty),*}
    }
}

unsafe impl UnsafePokable for *mut u8 {
    impl_ptr_write! {
        poke_i8: i8,
        poke_i16: i16,
        poke_i32: i32,
        poke_i64: i64,
        poke_u8: u8,
        poke_u16: u16,
        poke_u32: u32,
        poke_u64: u64,
        poke_f32: f32,
        poke_f64: f64
    }

    fn poke_bytes(self, v: &[u8]) -> Self {
        unsafe {
            ptr::copy_nonoverlapping(v.as_ptr(), self, v.len());
            self.add(v.len())
        }
    }
}

pub trait UnsafePoke {
    fn poke<UP: UnsafePokable>(&self, r: UP) -> UP;
    fn poke_max_size() -> usize;
}

impl UnsafePoke for () {
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        up
    }

    fn poke_max_size() -> usize {
        0
    }
}

macro_rules! impl_unsafe_poke_prim {
    ($ty:ty => $mem:ident as $cast:ty) => {
        impl_unsafe_poke_prim!{ __IMPL__ $ty, $mem, $cast }
    };
    ($ty:ty => $mem:ident) => {
        impl_unsafe_poke_prim!{ __IMPL__ $ty, $mem, $ty }
    };
    (__IMPL__ $ty:ty, $mem:ident, $cast:ty) => { 
        impl UnsafePoke for $ty {
            fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
                up.$mem(*self as $cast)
            }

            fn poke_max_size() -> usize {
                mem::size_of::<$cast>()
            }
        }
    }
}

impl_unsafe_poke_prim!( bool => poke_u8 as u8);
impl_unsafe_poke_prim!(isize => poke_i64 as i64);
impl_unsafe_poke_prim!(   i8 => poke_i8);
impl_unsafe_poke_prim!(  i16 => poke_i16);
impl_unsafe_poke_prim!(  i32 => poke_i32);
impl_unsafe_poke_prim!(  i64 => poke_i64);
impl_unsafe_poke_prim!(usize => poke_u64 as u64);
impl_unsafe_poke_prim!(   u8 => poke_u8);
impl_unsafe_poke_prim!(  u16 => poke_u16);
impl_unsafe_poke_prim!(  u32 => poke_u32);
impl_unsafe_poke_prim!(  u64 => poke_u64);
impl_unsafe_poke_prim!(  f32 => poke_f32);
impl_unsafe_poke_prim!(  f64 => poke_f64);

impl<T> UnsafePoke for Option<T>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
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
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        up
    }

    fn poke_max_size() -> usize {
        0
    }
}

macro_rules! impl_unsafe_poke_arrays {
    ($($len:tt)+) => {
        $(impl<T: UnsafePoke> UnsafePoke for [T; $len] {
            fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
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
            fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
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
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        up
    }

    fn poke_max_size() -> usize {
        0
    }
}

