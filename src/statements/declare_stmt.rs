use crate::memory::Memory;
use crate::types::*;
use crate::expr_tree::Eval;
use crate::statements::Statement;

#[derive(Debug)]
pub struct VarAssignment<T: Type> {
  pub var_id: usize,
  pub expr: Box<dyn Eval<T>>
}

impl<T: Type> VarAssignment<T> {
  pub fn new(var_id: usize, expr: Box<dyn Eval<T>>) -> VarAssignment<T> {
    VarAssignment {
      var_id,
      expr
    }
  }
}

impl Statement for VarAssignment<bool> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_bool(self.var_id, self.expr.eval(mem));
  }
}

impl Statement for VarAssignment<i64> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_i64(self.var_id, self.expr.eval(mem));
  }
}

impl Statement for VarAssignment<f64> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_f64(self.var_id, self.expr.eval(mem));
  }
}

impl Statement for VarAssignment<Str> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_str(self.var_id, self.expr.eval(mem));
  }
}