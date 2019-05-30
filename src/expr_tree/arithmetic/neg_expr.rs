use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct NegExpr<T: Type> {
  pub expr: Box<dyn Eval<T>>,
}

impl<T: Type> NegExpr<T> {
  pub fn new(expr: Box<dyn Eval<T>>) -> NegExpr<T> {
    NegExpr {
      expr
    }
  }
}

impl<T: Type> Eval<T> for NegExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.expr.eval(mem).neg()
  }
}
