use std::borrow::Cow;
use crate::types::Str;

pub trait OrSemantic<RHS=Self> {
  fn or(self, rhs: RHS) -> Self;
}

impl OrSemantic for bool {
  fn or(self, rhs: bool) -> Self {
    self || rhs
  }
}

impl OrSemantic for Str {
  fn or(self, rhs: Str) -> Self {
    unimplemented!()
  }
}

impl OrSemantic for i64 {
  fn or(self, rhs: i64) -> Self {
    unimplemented!()
  }
}

impl OrSemantic for f64 {
  fn or(self, rhs: f64) -> Self {
    unimplemented!()
  }
}

impl OrSemantic for Vec<Str> {
  fn or(self, rhs: Vec<Str>) -> Self {
    unimplemented!()
  }
}

impl OrSemantic for Vec<i64> {
  fn or(self, rhs: Vec<i64>) -> Self {
    unimplemented!()
  }
}

impl OrSemantic for Vec<f64> {
  fn or(self, rhs: Vec<f64>) -> Self {
    unimplemented!()
  }
}