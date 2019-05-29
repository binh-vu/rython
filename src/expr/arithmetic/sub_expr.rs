use std::marker::PhantomData;

use crate::memory::Memory;
use crate::types::*;

#[derive(Debug)]
pub struct SubExpr<T: Type> {
  pub left: usize,
  pub right: usize,
  phantom: PhantomData<T>,
}

impl<T: Type> SubExpr<T> {
  pub fn new(left: usize, right: usize) -> SubExpr<T> {
    SubExpr {
      left,
      right,
      phantom: PhantomData,
    }
  }
}

impl SubExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>) {
    registry[self.left] = registry[self.left] - registry[self.right]
  }
}

impl SubExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>) {
    registry[self.left] = registry[self.left] - registry[self.right]
  }
}