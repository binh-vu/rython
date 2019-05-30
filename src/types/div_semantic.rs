use std::borrow::Cow;
use crate::types::Str;

pub trait DivSemantic<RHS=Self> {
  fn div(self, rhs: RHS) -> Self;
}

impl DivSemantic for bool {
  fn div(self, rhs: bool) -> Self {
    unimplemented!()
  }
}


impl DivSemantic for Str {
  fn div(self, rhs: Str) -> Self {
    unimplemented!()
  }
}

impl DivSemantic for i64 {
  fn div(self, rhs: i64) -> Self {
    self / rhs
  }
}

impl DivSemantic for f64 {
  fn div(self, rhs: f64) -> Self {
    self / rhs
  }
}

impl DivSemantic for Vec<Str> {
  fn div(self, rhs: Vec<Str>) -> Self {
    unimplemented!()
  }
}

impl DivSemantic for Vec<i64> {
  fn div(self, rhs: Vec<i64>) -> Self {
    unimplemented!()
  }
}

impl DivSemantic for Vec<f64> {
  fn div(self, rhs: Vec<f64>) -> Self {
    unimplemented!()
  }
}