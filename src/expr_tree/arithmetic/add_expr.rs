use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct AddExpr<T: Type> {
  pub left: Box<dyn Eval<T>>,
  pub right: Box<dyn Eval<T>>,
}

impl<T: Type> AddExpr<T> {
  pub fn new(left: Box<dyn Eval<T>>, right: Box<dyn Eval<T>>) -> AddExpr<T> {
    AddExpr {
      left,
      right,
    }
  }
}

impl<T: Type> Eval<T> for AddExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.left.eval(mem).add(self.right.eval(mem))
  }
}
