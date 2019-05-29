use std::marker::PhantomData;

use crate::memory::Memory;
use crate::types::*;

#[derive(Debug)]
pub struct AddExpr<T: Type> {
  pub left: usize,
  pub right: usize,
  phantom: PhantomData<T>,
}

impl<T: Type> AddExpr<T> {
  pub fn new(left: usize, right: usize) -> AddExpr<T> {
    AddExpr {
      left,
      right,
      phantom: PhantomData,
    }
  }
}

impl AddExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>) {
    registry[self.left] = registry[self.left] + registry[self.right]
  }
}

impl AddExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>) {
    registry[self.left] = registry[self.left] + registry[self.right]
  }
}