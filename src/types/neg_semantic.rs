use std::borrow::Cow;
use crate::types::Str;

pub trait NegSemantic<RHS=Self> {
  fn neg(self) -> Self;
}

impl NegSemantic for bool {
  fn neg(self) -> Self {
    panic!("TypeError: cannot invoke neg on a bool type");
  }
}

impl NegSemantic for Str {
  fn neg(self) -> Self {
    panic!("TypeError: cannot invoke neg on a string type");
  }
}

impl NegSemantic for i64 {
  fn neg(self) -> Self {
    -self
  }
}

impl NegSemantic for f64 {
  fn neg(self) -> Self {
    -self
  }
}

impl NegSemantic for Vec<Str> {
  fn neg(self) -> Self {
    panic!("TypeError: cannot invoke neg on a List[str] type");
  }
}

impl NegSemantic for Vec<i64> {
  fn neg(self) -> Self {
    panic!("TypeError: cannot invoke neg on a List[int] type");
  }
}

impl NegSemantic for Vec<f64> {
  fn neg(self) -> Self {
    panic!("TypeError: cannot invoke neg on a List[float] type");
  }
}