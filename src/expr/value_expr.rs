use crate::types::*;
use std::marker::PhantomData;
use crate::memory::Memory;
use std::borrow::Cow;

#[derive(Debug)]
pub struct VarExpr<T: Type> {
  // registry id
  pub rid: usize,
  pub var_id: usize,
  phantom: PhantomData<T>,
}

impl<T: Type> VarExpr<T> {
  pub fn new(rid: usize, var_id: usize) -> VarExpr<T> {
    VarExpr { rid, var_id, phantom: PhantomData }
  }
}

impl VarExpr<i64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<i64>, mem: &Memory) {
    registry[self.rid] = mem.get_i64(self.var_id);
  }
}

impl VarExpr<f64> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<f64>, mem: &Memory) {
    registry[self.rid] = mem.get_f64(self.var_id);
  }
}

impl VarExpr<Str> {
  #[inline]
  pub fn eval(&self, registry: &mut Vec<Str>, mem: &Memory) {
    registry[self.rid] = mem.get_str(self.var_id);
  }
}
