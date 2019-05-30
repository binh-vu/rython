use crate::expr_tree::Eval;
use crate::memory::Memory;
use crate::types::*;

#[derive(Debug)]
pub struct VarExpr {
  pub var_id: usize,
}

impl Eval<i64> for VarExpr {
  fn eval(&mut self, mem: &Memory) -> i64 {
    mem.get_i64(self.var_id)
  }
}

impl Eval<Str> for VarExpr {
  fn eval(&mut self, mem: &Memory) -> Str {
    mem.get_str(self.var_id)
  }
}

impl Eval<f64> for VarExpr {
  fn eval(&mut self, mem: &Memory) -> f64 {
    mem.get_f64(self.var_id)
  }
}

impl Eval<bool> for VarExpr {
  fn eval(&mut self, mem: &Memory) -> bool {
    mem.get_bool(self.var_id)
  }
}