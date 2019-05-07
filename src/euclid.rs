use crate::{UnsafePokable, UnsafePoke};
use euclid::{
    TypedPoint2D, TypedRect, TypedSideOffsets2D, TypedSize2D, TypedTransform3D, TypedVector2D,
};
use std::mem::size_of;

impl<T, U> UnsafePoke for TypedPoint2D<T, U>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        let up = self.x.poke(up);
        let up = self.y.poke(up);
        up
    }

    fn poke_max_size() -> usize {
        2 * size_of::<T>()
    }
}

impl<T, U> UnsafePoke for TypedRect<T, U>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        let up = self.origin.poke(up);
        let up = self.size.poke(up);
        up
    }

    fn poke_max_size() -> usize {
        TypedPoint2D::<T, U>::poke_max_size() +
            TypedSize2D::<T, U>::poke_max_size()
    }
}

impl<T, U> UnsafePoke for TypedSideOffsets2D<T, U>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        let up = self.top.poke(up);
        let up = self.right.poke(up);
        let up = self.bottom.poke(up);
        let up = self.left.poke(up);
        up
    }

    fn poke_max_size() -> usize {
        4 * T::poke_max_size()
    }
}

impl<T, U> UnsafePoke for TypedSize2D<T, U>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        let up = self.width.poke(up);
        let up = self.height.poke(up);
        up
    }

    fn poke_max_size() -> usize {
        2 * T::poke_max_size()
    }
}

impl<T, Src, Dst> UnsafePoke for TypedTransform3D<T, Src, Dst>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        let up = self.m11.poke(up);
        let up = self.m12.poke(up);
        let up = self.m13.poke(up);
        let up = self.m14.poke(up);
        let up = self.m21.poke(up);
        let up = self.m22.poke(up);
        let up = self.m23.poke(up);
        let up = self.m24.poke(up);
        let up = self.m31.poke(up);
        let up = self.m32.poke(up);
        let up = self.m33.poke(up);
        let up = self.m34.poke(up);
        let up = self.m41.poke(up);
        let up = self.m42.poke(up);
        let up = self.m43.poke(up);
        let up = self.m44.poke(up);
        up
    }

    fn poke_max_size() -> usize {
        16 * T::poke_max_size()
    }
}

impl<T, U> UnsafePoke for TypedVector2D<T, U>
where
    T: UnsafePoke,
{
    fn poke<UP: UnsafePokable>(&self, up: UP) -> UP {
        let up = self.x.poke(up);
        let up = self.y.poke(up);
        up
    }

    fn poke_max_size() -> usize {
        2 * T::poke_max_size()
    }
}
