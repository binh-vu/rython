use crate::types::*;
use crate::memory::Memory;
use crate::expr::expr_enum::ExprEnum;
use super::Eval;

/// represent an expr, which value is computed and return while the intermediate values are dropped away
#[derive(Debug)]
pub struct Expr<T: Type> {
  pub registry: Vec<T>,
  pub commands: Vec<ExprEnum<T>>,
}

impl Eval<i64> for Expr<i64> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> i64 {
    for cmd in &mut self.commands {
      cmd.eval(&mut self.registry, mem);
    }
    self.registry[0].clone()
  }
}

impl Eval<f64> for Expr<f64> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> f64 {
    for cmd in &mut self.commands {
      cmd.eval(&mut self.registry, mem);
    }
    self.registry[0].clone()
  }
}

impl Eval<Str> for Expr<Str> {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> Str {
    for cmd in &mut self.commands {
      cmd.eval(&mut self.registry, mem);
    }
    self.registry[0].clone()
  }
}