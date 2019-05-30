use crate::statements::{Statement, StatementType};
use crate::expr_tree::Eval;
use crate::memory::Memory;
use crate::types::Type;

#[derive(Debug)]
pub struct IfFalseGoto {
  pub condition: Box<dyn Eval<bool>>,
  pub pc: usize
}

impl Statement for IfFalseGoto {
  fn get_type(&self) -> StatementType {
    StatementType::Goto
  }

  fn exec(&mut self, mem: &mut Memory) {
    // jump if false
    if !self.condition.eval(mem) {
      mem.pc = self.pc;
    }
  }
}
