use std::borrow::Cow;
use std::fmt::Debug;

pub use self::add_semantic::AddSemantic;
pub use self::sub_semantic::SubSemantic;
pub use self::mul_semantic::MulSemantic;
pub use self::div_semantic::DivSemantic;
pub use self::neg_semantic::NegSemantic;
pub use self::and_semantic::AndSemantic;
pub use self::or_semantic::OrSemantic;
pub use self::not_semantic::NotSemantic;
pub use self::var_type::*;

pub mod var_type;
pub mod add_semantic;
pub mod sub_semantic;
pub mod mul_semantic;
pub mod div_semantic;
pub mod neg_semantic;
pub mod and_semantic;
pub mod or_semantic;
pub mod not_semantic;

pub type Str = Cow<'static, String>;

pub trait Type: Default + Debug + Clone + AddSemantic + SubSemantic + DivSemantic + MulSemantic + NegSemantic + AndSemantic + OrSemantic + NotSemantic {}

impl Type for bool {}

impl Type for Str {}

impl Type for i64 {}

impl Type for f64 {}

impl Type for Vec<Str> {}

impl Type for Vec<i64> {}

impl Type for Vec<f64> {}

#[derive(Debug)]
pub enum Value {
  Str(Str),
  I64(i64),
  F64(f64),
  Bool(bool),
}

impl Value {
  pub fn as_i64(&self) -> i64 {
    match self {
      Value::I64(v) => *v,
      _ => panic!("[BUG]")
    }
  }

  pub fn as_f64(&self) -> f64 {
    match self {
      Value::F64(v) => *v,
      _ => panic!("[BUG]")
    }
  }

  pub fn as_str(&self) -> Str {
    match self {
      Value::Str(v) => v.clone(),
      _ => panic!("[BUG]")
    }
  }

  pub fn as_bool(&self) -> bool {
    match self {
      Value::Bool(v) => v.clone(),
      _ => panic!("[BUG]")
    }
  }
}