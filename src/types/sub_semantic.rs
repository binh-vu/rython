use std::borrow::Cow;
use crate::types::Str;

pub trait SubSemantic<RHS=Self> {
  fn sub(self, rhs: RHS) -> Self;
}

impl SubSemantic for bool {
  fn sub(self, rhs: bool) -> Self {
    unimplemented!()
  }
}

impl SubSemantic for Str {
  fn sub(self, rhs: Str) -> Self {
    unimplemented!()
  }
}

impl SubSemantic for i64 {
  fn sub(self, rhs: i64) -> Self {
    self - rhs
  }
}

impl SubSemantic for f64 {
  fn sub(self, rhs: f64) -> Self {
    self - rhs
  }
}

impl SubSemantic for Vec<Str> {
  fn sub(self, rhs: Vec<Str>) -> Self {
    unimplemented!()
  }
}

impl SubSemantic for Vec<i64> {
  fn sub(self, rhs: Vec<i64>) -> Self {
    unimplemented!()
  }
}

impl SubSemantic for Vec<f64> {
  fn sub(self, rhs: Vec<f64>) -> Self {
    unimplemented!()
  }
}