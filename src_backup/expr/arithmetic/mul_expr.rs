use std::marker::PhantomData;

use crate::memory::Memory;
use crate::types::*;

#[derive(Debug)]
pub struct MulExpr<T: Type> {
  pub left: usize,
  pub right: usize,
  phantom: PhantomData<T>,
}

impl<T: Type> MulExpr<T> {
  pub fn new(left: usize, right: usize) -> MulExpr<T> {
    MulExpr {
      left,
      right,
      phantom: PhantomData,
    }
  }
}

impl MulExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>) {
    registry[self.left] = registry[self.left] * registry[self.right]
  }
}

impl MulExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>) {
    registry[self.left] = registry[self.left] * registry[self.right]
  }
}