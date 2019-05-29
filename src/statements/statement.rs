use crate::statements::declare_stmt::DefVar;
use crate::types::*;

#[derive(Debug)]
pub enum Statement {
  DefVarI64(DefVar<i64>),
  DefVarF64(DefVar<f64>),
  DefVarStr(DefVar<Str>),
}
