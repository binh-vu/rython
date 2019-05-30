use crate::expr::expr::Expr;
use crate::memory::Memory;
use crate::types::*;
use crate::expr::Eval;

#[derive(Debug)]
pub struct DefVar<T: Type> {
  pub var_id: usize,
  pub expr: Expr<T>
}

impl<T: Type> DefVar<T> {
  pub fn new(var_id: usize, expr: Expr<T>) -> DefVar<T> {
    DefVar {
      var_id,
      expr
    }
  }
}

impl DefVar<i64> {
  pub fn exec(&mut self, mem: &mut Memory) {
    mem.set_i64(self.var_id, self.expr.eval(mem));
  }
}

impl DefVar<f64> {
  pub fn exec(&mut self, mem: &mut Memory) {
    mem.set_f64(self.var_id, self.expr.eval(mem));
  }
}

impl DefVar<Str> {
  pub fn exec(&mut self, mem: &mut Memory) {
    mem.set_str(self.var_id, self.expr.eval(mem));
  }
}