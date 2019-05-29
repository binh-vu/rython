use crate::expr::Eval;
use crate::memory::Memory;

#[derive(Debug)]
pub struct MathCeil {
  arg: Eval<f64>,
}

impl Eval<f64> for MathCeil {
  fn eval(&mut self, mem: &Memory) -> f64 { self.arg.eval(mem).ceil() }
}

#[derive(Debug)]
pub struct MathFAbs {
  arg: Eval<f64>,
}

impl Eval<f64> for MathFAbs {
  fn eval(&mut self, mem: &Memory) -> f64 { self.arg.eval(mem).abs() }
}

#[derive(Debug)]
pub struct MathExp {
  arg: Eval<f64>,
}

impl Eval<f64> for MathExp {
  fn eval(&mut self, mem: &Memory) -> f64 { self.arg.eval(mem).exp() }
}