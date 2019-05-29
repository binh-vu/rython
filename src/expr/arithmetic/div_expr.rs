use std::marker::PhantomData;

use crate::memory::Memory;
use crate::types::*;

#[derive(Debug)]
pub struct DivExpr<T: Type> {
  pub left: usize,
  pub right: usize,
  phantom: PhantomData<T>,
}

impl<T: Type> DivExpr<T> {
  pub fn new(left: usize, right: usize) -> DivExpr<T> {
    DivExpr {
      left,
      right,
      phantom: PhantomData,
    }
  }
}

impl DivExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>) {
    registry[self.left] = registry[self.left] / registry[self.right]
  }
}

impl DivExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>) {
    registry[self.left] = registry[self.left] / registry[self.right]
  }
}