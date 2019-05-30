use crate::memory::Memory;
use crate::statements::statement::Statement;

#[derive(Debug)]
pub struct Program {
  pub memory: Memory,
  pub statements: Vec<Statement>,
}
