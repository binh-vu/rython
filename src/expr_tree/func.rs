use crate::expr_tree::{WrappedEval, Eval};
use crate::memory::Memory;
use crate::types::Str;

#[derive(Debug)]
pub struct FuncExpr {
  pub name: String,
  pub args: Vec<WrappedEval>
}

impl Eval<i64> for FuncExpr {
  fn eval(&mut self, mem: &Memory) -> i64 {
    unimplemented!()
  }
}

impl Eval<f64> for FuncExpr {
  fn eval(&mut self, mem: &Memory) -> f64 {
    unimplemented!()
  }
}

impl Eval<Str> for FuncExpr {
  fn eval(&mut self, mem: &Memory) -> Str {
    unimplemented!()
  }
}

impl Eval<bool> for FuncExpr {
  fn eval(&mut self, mem: &Memory) -> bool {
    unimplemented!()
  }
}