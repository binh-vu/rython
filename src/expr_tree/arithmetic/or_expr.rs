use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct OrExpr<T: Type> {
  pub left: Box<dyn Eval<T>>,
  pub right: Box<dyn Eval<T>>,
}

impl<T: Type> OrExpr<T> {
  pub fn new(left: Box<dyn Eval<T>>, right: Box<dyn Eval<T>>) -> OrExpr<T> {
    OrExpr {
      left,
      right,
    }
  }
}

impl<T: Type> Eval<T> for OrExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.left.eval(mem).or(self.right.eval(mem))
  }
}
