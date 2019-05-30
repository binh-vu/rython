use std::fmt::Debug;

use crate::memory::Memory;
use crate::types::*;

pub use self::arithmetic::*;
pub use self::constant::*;
pub use self::variable::*;
pub use self::func::*;

pub mod variable;
pub mod constant;
pub mod func;
pub mod arithmetic;

/// represent something that can be evaluate to get the result of type R
pub trait Eval<R: Type>: Debug {
  #[inline]
  fn eval(&mut self, mem: &Memory) -> R;
}

#[derive(Debug)]
pub enum WrappedEval {
  I64(Box<dyn Eval<i64>>),
  F64(Box<dyn Eval<f64>>),
  Str(Box<dyn Eval<Str>>),
  Bool(Box<dyn Eval<bool>>),
}