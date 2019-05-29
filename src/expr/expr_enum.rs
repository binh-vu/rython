use crate::types::*;
use crate::memory::Memory;
use super::arithmetic::*;
use super::constant_expr::ConstantExpr;
use super::value_expr::VarExpr;
use crate::expr::func::FuncExpr;

#[derive(Debug)]
pub enum ExprEnum<T: Type> {
  Neg(NegExpr<T>),
  Add(AddExpr<T>),
  Sub(SubExpr<T>),
  Mul(MulExpr<T>),
  Div(DivExpr<T>),
  Constant(ConstantExpr<T>),
  Var(VarExpr<T>),
  Func(FuncExpr<T>)
}

impl ExprEnum<i64> {
  #[inline]
  pub fn eval(&mut self, registry: &mut Vec<i64>, mem: &Memory) {
    match self {
      ExprEnum::Neg(cmd) => cmd.eval(registry),
      ExprEnum::Add(cmd) => cmd.eval(registry),
      ExprEnum::Sub(cmd) => cmd.eval(registry),
      ExprEnum::Mul(cmd) => cmd.eval(registry),
      ExprEnum::Div(cmd) => cmd.eval(registry),
      ExprEnum::Constant(cmd) => cmd.eval(registry),
      ExprEnum::Var(cmd) => cmd.eval(registry, mem),
      ExprEnum::Func(func) => func.eval(registry, mem)
    }
  }
}

impl ExprEnum<f64> {
  #[inline]
  pub fn eval(&mut self, registry: &mut Vec<f64>, mem: &Memory) {
    match self {
      ExprEnum::Neg(cmd) => cmd.eval(registry),
      ExprEnum::Add(cmd) => cmd.eval(registry),
      ExprEnum::Sub(cmd) => cmd.eval(registry),
      ExprEnum::Mul(cmd) => cmd.eval(registry),
      ExprEnum::Div(cmd) => cmd.eval(registry),
      ExprEnum::Constant(cmd) => cmd.eval(registry),
      ExprEnum::Var(cmd) => cmd.eval(registry, mem),
      ExprEnum::Func(func) => func.eval(registry, mem)
    }
  }
}

impl ExprEnum<Str> {
  #[inline]
  pub fn eval(&mut self, registry: &mut Vec<Str>, mem: &Memory) {
    match self {
      ExprEnum::Neg(_) => panic!("Type mismatch"),
      ExprEnum::Add(_) => panic!("Type mismatch"),
      ExprEnum::Sub(_) => panic!("Type mismatch"),
      ExprEnum::Mul(_) => panic!("Type mismatch"),
      ExprEnum::Div(_) => panic!("Type mismatch"),
      ExprEnum::Constant(cmd) => cmd.eval(registry),
      ExprEnum::Var(cmd) => cmd.eval(registry, mem),
      ExprEnum::Func(func) => func.eval(registry, mem)
    }
  }
}
