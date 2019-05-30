use crate::memory::Memory;
use crate::types::*;

use super::Eval;

#[derive(Debug)]
pub struct ConstantExpr<T> {
  pub val: T
}

impl<T: Type> Eval<T> for ConstantExpr<T> {
  fn eval(&mut self, mem: &Memory) -> T {
    self.val.clone()
  }
}
