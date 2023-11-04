use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign};
use std::fmt::{Display, Debug};

pub trait Number:
    Sized + Clone + Copy +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign +
    Display + Debug +
    PartialEq + PartialOrd +
    Default
{
}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}
impl Number for f32 {}
impl Number for f64 {}
