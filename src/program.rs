use crate::memory::Memory;
use crate::statements::Statement;
use crate::types::{VarType, Value, SingleType};

#[derive(Debug)]
pub struct Program {
  pub memory: Memory,
  pub return_var_id: usize,
  pub return_type: VarType,
  pub statements: Vec<Box<dyn Statement>>,
}


impl Program {
  pub fn exec(&mut self) -> Value {
    while self.memory.pc < self.statements.len() {
      self.statements[self.memory.pc].exec(&mut self.memory);
      self.memory.pc += 1;
    }

    match &self.return_type {
      VarType::Single(t) => {
        match t {
          SingleType::Bool => {
            Value::Bool(self.memory.get_bool(self.return_var_id))
          },
          SingleType::Str => {
            Value::Str(self.memory.get_str(self.return_var_id))
          }
          SingleType::I64 => {
            Value::I64(self.memory.get_i64(self.return_var_id))
          }
          SingleType::F64 => {
            Value::F64(self.memory.get_f64(self.return_var_id))
          }
        }
      },
      _ => unimplemented!()
    }
  }
}