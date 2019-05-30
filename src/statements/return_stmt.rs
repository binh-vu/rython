use crate::expr_tree::Eval;
use crate::types::{Type, Str};
use crate::statements::Statement;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ReturnStmt<T: Type> {
  pub var_id: usize,
  pub expr: Box<dyn Eval<T>>
}

impl Statement for ReturnStmt<bool> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_bool(self.var_id, self.expr.eval(mem));
  }
}

impl Statement for ReturnStmt<i64> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_i64(self.var_id, self.expr.eval(mem));
  }
}

impl Statement for ReturnStmt<f64> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_f64(self.var_id, self.expr.eval(mem));
  }
}

impl Statement for ReturnStmt<Str> {
  fn exec(&mut self, mem: &mut Memory) {
    mem.set_str(self.var_id, self.expr.eval(mem));
  }
}