use std::borrow::Cow;
use crate::types::Str;

pub trait AddSemantic<RHS=Self> {
  fn add(self, rhs: RHS) -> Self;
}

impl AddSemantic for bool {
  fn add(self, rhs: bool) -> Self {
    unimplemented!()
  }
}

impl AddSemantic for Str {
  fn add(self, rhs: Str) -> Self {
    Cow::Owned(format!("{}{}", self, rhs))
  }
}

impl AddSemantic for i64 {
  fn add(self, rhs: i64) -> Self {
    self + rhs
  }
}

impl AddSemantic for f64 {
  fn add(self, rhs: f64) -> Self {
    self + rhs
  }
}

impl AddSemantic for Vec<Str> {
  fn add(self, rhs: Vec<Str>) -> Self {
    unimplemented!()
  }
}

impl AddSemantic for Vec<i64> {
  fn add(self, rhs: Vec<i64>) -> Self {
    unimplemented!()
  }
}

impl AddSemantic for Vec<f64> {
  fn add(self, rhs: Vec<f64>) -> Self {
    unimplemented!()
  }
}