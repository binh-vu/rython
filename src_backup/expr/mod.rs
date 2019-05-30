use crate::types::Type;
use crate::memory::Memory;
use std::fmt::Debug;

pub mod expr;
pub mod expr_enum;
pub mod value_expr;
pub mod constant_expr;
pub mod func;
pub mod arithmetic;

pub trait Eval<R: Type>: Debug {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> R;
}