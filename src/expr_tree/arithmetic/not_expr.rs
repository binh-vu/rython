use crate::memory::Memory;
use crate::types::*;

use crate::expr_tree::Eval;

#[derive(Debug)]
pub struct NotExpr<T: Type> {
  pub expr: Box<dyn Eval<T>>,
}

impl<T: Type> NotExpr<T> {
  pub fn new(expr: Box<dyn Eval<T>>) -> NotExpr<T> {
    NotExpr {
      expr
    }
  }
}

impl<T: Type> Eval<T> for NotExpr<T> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> T {
    self.expr.eval(mem).not()
  }
}
