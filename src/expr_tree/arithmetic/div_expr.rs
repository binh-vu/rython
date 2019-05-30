use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct DivExpr<T: Type> {
  pub left: Box<dyn Eval<T>>,
  pub right: Box<dyn Eval<T>>,
}

impl<T: Type> DivExpr<T> {
  pub fn new(left: Box<dyn Eval<T>>, right: Box<dyn Eval<T>>) -> DivExpr<T> {
    DivExpr {
      left,
      right,
    }
  }
}

impl<T: Type> Eval<T> for DivExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.left.eval(mem).div(self.right.eval(mem))
  }
}
