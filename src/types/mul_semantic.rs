use std::borrow::Cow;
use crate::types::Str;

pub trait MulSemantic<RHS=Self> {
  fn mul(self, rhs: RHS) -> Self;
}

impl MulSemantic for bool {
  fn mul(self, rhs: bool) -> Self {
    unimplemented!()
  }
}

impl MulSemantic for Str {
  fn mul(self, rhs: Str) -> Self {
    unimplemented!()
  }
}

impl MulSemantic for i64 {
  fn mul(self, rhs: i64) -> Self {
    self * rhs
  }
}

impl MulSemantic for f64 {
  fn mul(self, rhs: f64) -> Self {
    self * rhs
  }
}

impl MulSemantic for Vec<Str> {
  fn mul(self, rhs: Vec<Str>) -> Self {
    unimplemented!()
  }
}

impl MulSemantic for Vec<i64> {
  fn mul(self, rhs: Vec<i64>) -> Self {
    unimplemented!()
  }
}

impl MulSemantic for Vec<f64> {
  fn mul(self, rhs: Vec<f64>) -> Self {
    unimplemented!()
  }
}