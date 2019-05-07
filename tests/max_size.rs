#![allow(dead_code)]
use std::{marker::PhantomData, mem::size_of};
use unsafe_poke::UnsafePoke;
use unsafe_poke_derive::UnsafePoke;

#[test]
fn test_numbers() {
    assert_eq!(u8::poke_max_size(), size_of::<u8>());
    assert_eq!(u16::poke_max_size(), size_of::<u16>());
    assert_eq!(u32::poke_max_size(), size_of::<u32>());
    assert_eq!(u64::poke_max_size(), size_of::<u64>());
    assert_eq!(usize::poke_max_size(), size_of::<usize>());
    assert_eq!(i8::poke_max_size(), size_of::<i8>());
    assert_eq!(i16::poke_max_size(), size_of::<i16>());
    assert_eq!(i32::poke_max_size(), size_of::<i32>());
    assert_eq!(i64::poke_max_size(), size_of::<i64>());
    assert_eq!(isize::poke_max_size(), size_of::<isize>());
    // floating
    assert_eq!(f32::poke_max_size(), size_of::<f32>());
    assert_eq!(f64::poke_max_size(), size_of::<f64>());
}

#[test]
fn test_bool() {
    assert_eq!(bool::poke_max_size(), size_of::<u8>());
}

#[test]
fn test_option() {
    assert_eq!(Option::<usize>::poke_max_size(), 1 + size_of::<usize>());
}

#[test]
fn test_fixed_size_array() {
    assert_eq!(<[u32; 32]>::poke_max_size(), 32 * size_of::<u32>());
    assert_eq!(<[u64; 8]>::poke_max_size(), 8 * size_of::<u64>());
    assert_eq!(<[u8; 19]>::poke_max_size(), 19 * size_of::<u8>());
}

#[test]
fn test_tuple() {
    assert_eq!(<(isize)>::poke_max_size(), size_of::<isize>());
    assert_eq!(
        <(isize, isize, isize)>::poke_max_size(),
        3 * size_of::<isize>()
    );
    assert_eq!(<(isize, ())>::poke_max_size(), size_of::<isize>());
}

#[test]
fn test_basic_struct() {
    #[derive(Debug, UnsafePoke)]
    struct Bar {
        a: u32,
        b: u32,
        c: u32,
        d: Option<u32>,
    }

    assert_eq!(Bar::poke_max_size(), 4 * size_of::<u32>() + 1);
}

#[test]
fn test_enum() {
    #[derive(UnsafePoke)]
    enum TestEnum {
        NoArg,
        OneArg(usize),
        Args(usize, usize),
        AnotherNoArg,
        StructLike { x: usize, y: f32 },
    }
    assert_eq!(
        TestEnum::poke_max_size(),
        size_of::<u32>() + 2 * size_of::<usize>()
    );
}

#[test]
fn test_enum_cstyle() {
    #[repr(u32)]
    #[derive(Clone, Copy, UnsafePoke)]
    enum BorderStyle {
        None = 0,
        Solid = 1,
        Double = 2,
        Dotted = 3,
        Dashed = 4,
        Hidden = 5,
        Groove = 6,
        Ridge = 7,
        Inset = 8,
        Outset = 9,
    }
    assert_eq!(BorderStyle::poke_max_size(), size_of::<u32>());
}

#[test]
fn test_phantom_data() {
    struct Bar;
    #[derive(UnsafePoke)]
    struct Foo {
        x: u32,
        y: u32,
        _marker: PhantomData<Bar>,
    }
    assert_eq!(Foo::poke_max_size(), 2 * size_of::<u32>())
}
