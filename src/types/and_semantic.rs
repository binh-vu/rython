use std::borrow::Cow;
use crate::types::Str;

pub trait AndSemantic<RHS=Self> {
  fn and(self, rhs: RHS) -> Self;
}

impl AndSemantic for bool {
  fn and(self, rhs: bool) -> Self {
    self && rhs
  }
}

impl AndSemantic for Str {
  fn and(self, rhs: Str) -> Self {
    unimplemented!()
  }
}

impl AndSemantic for i64 {
  fn and(self, rhs: i64) -> Self {
    unimplemented!()
  }
}

impl AndSemantic for f64 {
  fn and(self, rhs: f64) -> Self {
    unimplemented!()
  }
}

impl AndSemantic for Vec<Str> {
  fn and(self, rhs: Vec<Str>) -> Self {
    unimplemented!()
  }
}

impl AndSemantic for Vec<i64> {
  fn and(self, rhs: Vec<i64>) -> Self {
    unimplemented!()
  }
}

impl AndSemantic for Vec<f64> {
  fn and(self, rhs: Vec<f64>) -> Self {
    unimplemented!()
  }
}