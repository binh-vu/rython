use crate::statements::{Statement, StatementType};
use crate::memory::Memory;

#[derive(Debug)]
pub struct Goto {
  pub pc: usize
}

impl Statement for Goto {
  fn get_type(&self) -> StatementType {
    StatementType::Goto
  }

  fn exec(&mut self, mem: &mut Memory) {
    mem.pc = self.pc;
  }
}
