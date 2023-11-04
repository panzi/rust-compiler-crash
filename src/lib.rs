#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod number;
pub mod assert;

use std::marker::PhantomData;

use crate::assert::{Assert, IsTrue};
#[cfg(any(feature="crash2",feature="crash6"))]
use crate::assert::TypeEq;
use crate::number::Number;

#[repr(transparent)]
#[derive(Clone)]
pub struct Matrix<const X: usize, const Y: usize, T: Number=f64, D: AsRef<[T; X * Y]>=Box<[T; X * Y]>>
where [T; X * Y]: Sized
{
    data: D,
    phantom_data: PhantomData::<T>,
}

impl<const X: usize, const Y: usize, T: Number, D: AsRef<[T; X * Y]>> Matrix<X, Y, T, D>
where [T; X * Y]: Sized
{
    #[inline]
    pub fn new(data: D) -> Self {
        Self { data, phantom_data: PhantomData::<T> }
    }

    #[inline]
    pub fn data(&self) -> &[T; X * Y] {
        self.data.as_ref()
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut [T; X * Y]
    where D: AsMut<[T; X * Y]> {
        self.data.as_mut()
    }

    #[cfg(feature="crash1")]
    #[inline]
    pub fn reshape<const X2: usize, const Y2: usize>(&self) -> Matrix<X2, Y2, T>
    where [T; X2 * Y2]: Sized, Assert<{ X * Y == X2 * Y2 }>: IsTrue {
        Matrix::new(self.data().clone())
    }

    #[cfg(feature="crash2")]
    #[inline]
    pub fn into_reshape<const X2: usize, const Y2: usize>(self) -> Matrix<X2, Y2, T>
    where [T; X2 * Y2]: Sized, Assert<{ X * Y == X2 * Y2 }>: IsTrue, TypeEq<D, Box<[T; X * Y]>>: IsTrue {
        unsafe { std::mem::transmute(self) }
    }

    #[cfg(feature="crash3")]
    #[inline]
    pub fn map<F, U>(&self, f: F) -> Matrix<X, Y, U>
    where F: FnMut(T) -> U, U: Number {
        Matrix { data: Box::new(self.data.map(f)), phantom_data: PhantomData::<T> }
    }

    #[cfg(feature="crash4")]
    pub fn transpose(&self) -> Matrix<Y, X, T>
    where [T; Y * X]: Sized {
        let mut data = Box::new([T::default(); Y * X]);

        for y in 0..Y {
            let yoffset = y * X;
            for x in 0..X {
                data[x * Y + y] = self.data()[yoffset + x];
            }
        }

        Matrix { data, phantom_data: PhantomData::<T> }
    }

    #[cfg(feature="crash5")]
    pub fn transpose_map<F, U>(&self, mut f: F) -> Matrix<Y, X, U>
    where F: FnMut(T) -> U, U: Number, [T; Y * X]: Sized {
        let mut data = Box::new([U::default(); Y * X]);

        for y in 0..Y {
            let yoffset = y * X;
            for x in 0..X {
                data[x * Y + y] = f(self.data()[yoffset + x]);
            }
        }

        Matrix::from(data)
    }

    #[inline]
    pub fn transpose_assign(&mut self)
    where [T; Y * X]: Sized, Assert<{ X == Y }>: IsTrue, D: AsMut<[T; X * Y]> {
        let data = self.data_mut();
        for y in 1..Y {
            let yoffset = y * X;
            for x in 0..y {
                data.swap(yoffset + x, x * X + y);
            }
        }
    }

    #[inline]
    #[cfg(feature="crash6")]
    pub fn into_transpose(mut self) -> Matrix<Y, X, T>
    where [T; Y * X]: Sized, Assert<{ X == Y }>: IsTrue, TypeEq<D, Box<[T; X * Y]>>: IsTrue {
        self.transpose_assign();
        // XXX: the compiler doesn't understand that X == Y
        unsafe { std::mem::transmute(self) }
    }
}
