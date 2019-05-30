use std::borrow::Cow;
use crate::types::Str;

pub trait NotSemantic<RHS=Self> {
  fn not(self) -> Self;
}

impl NotSemantic for bool {
  fn not(self) -> Self {
    !self
  }
}

impl NotSemantic for Str {
  fn not(self) -> Self {
    unimplemented!()
  }
}

impl NotSemantic for i64 {
  fn not(self) -> Self {
    unimplemented!()
  }
}

impl NotSemantic for f64 {
  fn not(self) -> Self {
    unimplemented!()
  }
}

impl NotSemantic for Vec<Str> {
  fn not(self) -> Self {
    unimplemented!()
  }
}

impl NotSemantic for Vec<i64> {
  fn not(self) -> Self {
    unimplemented!()
  }
}

impl NotSemantic for Vec<f64> {
  fn not(self) -> Self {
    unimplemented!()
  }
}