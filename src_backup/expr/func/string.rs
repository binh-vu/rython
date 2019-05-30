use crate::expr::Eval;
use crate::memory::Memory;
use crate::types::Str;

use std::borrow::Cow;

#[derive(Debug)]
pub struct StringLower {
  arg: Eval<Str>
}

impl Eval<Str> for StringLower {
  fn eval(&mut self, mem: &Memory) -> Str { Cow::Owned(self.arg.eval(mem).to_lowercase()) }
}