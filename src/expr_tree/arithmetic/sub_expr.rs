use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct SubExpr<T: Type> {
  pub left: Box<dyn Eval<T>>,
  pub right: Box<dyn Eval<T>>,
}

impl<T: Type> SubExpr<T> {
  pub fn new(left: Box<dyn Eval<T>>, right: Box<dyn Eval<T>>) -> SubExpr<T> {
    SubExpr {
      left,
      right,
    }
  }
}

impl<T: Type> Eval<T> for SubExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.left.eval(mem).sub(self.right.eval(mem))
  }
}
