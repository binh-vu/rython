use crate::types::Type;
use crate::expr::Eval;
use crate::memory::Memory;

pub mod math;
pub mod string;

#[derive(Debug)]
pub struct FuncExpr<T: Type> {
  pub rid: usize,
  pub func: Box<dyn Eval<T>>
}

impl<T: Type> FuncExpr<T> {
  #[inline]
  pub fn eval(&mut self, registry: &mut Vec<T>, mem: &Memory) {
    registry[self.rid] = self.func.eval(mem);
  }
}