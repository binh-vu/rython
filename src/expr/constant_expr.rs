use std::marker::PhantomData;

use crate::types::*;

#[derive(Debug)]
pub enum Constant {
  Str(Str),
  I64(i64),
  F64(f64),
}

impl Constant {
  pub fn as_i64(&self) -> i64 {
    match self {
      Constant::I64(v) => *v,
      _ => panic!("[BUG]")
    }
  }

  pub fn as_f64(&self) -> f64 {
    match self {
      Constant::F64(v) => *v,
      _ => panic!("[BUG]")
    }
  }

  pub fn as_str(&self) -> Str {
    match self {
      Constant::Str(v) => v.clone(),
      _ => panic!("[BUG]")
    }
  }
}

#[derive(Debug)]
pub struct ConstantExpr<T: Type> {
  // registry id
  pub rid: usize,
  pub val: Constant,
  phantom: PhantomData<T>,
}

impl<T: Type> ConstantExpr<T> {
  pub fn new(rid: usize, val: Constant) -> ConstantExpr<T> {
    ConstantExpr { rid, val, phantom: PhantomData }
  }
}

impl ConstantExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>) {
    registry[self.rid] = self.val.as_i64();
  }
}

impl ConstantExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>) {
    registry[self.rid] = self.val.as_f64();
  }
}

impl ConstantExpr<Str> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<Str>) {
    registry[self.rid] = self.val.as_str();
  }
}