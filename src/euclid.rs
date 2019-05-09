use crate::UnsafePod;
use euclid::{
    TypedPoint2D, TypedRect, TypedSideOffsets2D, TypedSize2D, TypedTransform3D, TypedVector2D,
};

unsafe impl<T, U> UnsafePod for TypedPoint2D<T, U> where T: Copy {}
unsafe impl<T, U> UnsafePod for TypedRect<T, U> where T: Copy {}
unsafe impl<T, U> UnsafePod for TypedSideOffsets2D<T, U> where T: Copy {}
unsafe impl<T, U> UnsafePod for TypedSize2D<T, U> where T: Copy {}
unsafe impl<T, S, D> UnsafePod for TypedTransform3D<T, S, D> where T: Copy {}
unsafe impl<T, U> UnsafePod for TypedVector2D<T, U> where T: Copy {}
