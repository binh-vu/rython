use crate::statements::declare_stmt::VarAssignment;
use crate::types::*;
use crate::memory::Memory;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum StatementType {
  NonGoto,
  Goto,
  Placeholder(usize)
}

pub trait Statement: Debug {
  fn get_type(&self) -> StatementType {
    StatementType::NonGoto
  }

  fn exec(&mut self, mem: &mut Memory);
}

#[derive(Debug)]
pub struct Placeholder {
  pub level: usize
}

impl Placeholder {
  pub fn new(level: usize) -> Placeholder {
    Placeholder { level }
  }
}

impl Statement for Placeholder {
  fn get_type(&self) -> StatementType {
    StatementType::Placeholder(self.level)
  }

  fn exec(&mut self, mem: &mut Memory) {
    unimplemented!()
  }
}
