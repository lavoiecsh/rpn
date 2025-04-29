mod integer;
mod float;

use core::fmt::{Debug, Display};
use core::ops::{Add, Div, Mul, Sub};

pub trait Number:
Sized
+ Copy
+ Clone
+ Debug
+ Display
+ Add<Output = Self>
+ Sub<Output = Self>
+ Mul<Output = Self>
+ Div<Output = Self>
{
    fn zero() -> Self;
}
