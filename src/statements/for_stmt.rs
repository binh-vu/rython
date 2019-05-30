use crate::types::Type;
use crate::statements::Statement;
use crate::memory::Memory;

#[derive(Debug)]
pub struct ForGoto {
  iter_id: usize,
  pc: usize
}

//impl Statement for ForGoto<i64> {
//  fn exec(&mut self, mem: &mut Memory) {
//  }
//}