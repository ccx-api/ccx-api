use std::iter::{empty, Product, Sum};
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Seq<T>(T);

impl<T> Seq<T>
where
    T: Sum<T>,
    T: Product<T>,
    T: AddAssign<T>,
    T: Copy,
{
    pub fn new() -> Self {
        Seq(empty().sum())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> T {
        self.0 += empty().product();
        self.0
    }
}

impl<T> Default for Seq<T>
where
    T: Sum<T>,
    T: Product<T>,
    T: AddAssign<T>,
    T: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
