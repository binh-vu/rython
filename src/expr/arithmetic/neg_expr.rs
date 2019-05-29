use crate::types::*;
use std::marker::PhantomData;
use crate::memory::Memory;

#[derive(Debug)]
pub struct NegExpr<T: Type> {
  pub expr: usize,
  phantom: PhantomData<T>
}

impl<T: Type> NegExpr<T> {
  pub fn new(expr: usize) -> NegExpr<T> {
    NegExpr {
      expr,
      phantom: PhantomData
    }
  }
}

impl NegExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>) {
    registry[self.expr] = -registry[self.expr];
  }
}

impl NegExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>) {
    registry[self.expr] = -registry[self.expr];
  }
}