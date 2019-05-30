use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct AndExpr<T: Type> {
  pub left: Box<dyn Eval<T>>,
  pub right: Box<dyn Eval<T>>,
}

impl<T: Type> AndExpr<T> {
  pub fn new(left: Box<dyn Eval<T>>, right: Box<dyn Eval<T>>) -> AndExpr<T> {
    AndExpr {
      left,
      right,
    }
  }
}

impl<T: Type> Eval<T> for AndExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.left.eval(mem).and(self.right.eval(mem))
  }
}
