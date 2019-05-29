use std::borrow::Cow;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum SingleType {
  Str,
  I64,
  F64,
}

#[derive(Debug, Clone)]
pub enum VarType {
  Single(SingleType),
  List(SingleType),
  Dict((SingleType, SingleType)),
}

pub type Str = Cow<'static, String>;

pub trait ArithmeticType: Default + Clone {}
pub trait Type: Default + Clone {}

impl ArithmeticType for i64 {}
impl ArithmeticType for f64 {}

impl Type for Str {}
impl Type for i64 {}
impl Type for f64 {}
impl Type for Vec<Str> {}
impl Type for Vec<i64> {}
impl Type for Vec<f64> {}

impl VarType {
  pub fn is_number(&self) -> bool {
    match self {
      VarType::Single(t) => {
        match t {
          SingleType::Str => false,
          SingleType::I64 => true,
          SingleType::F64 => true
        }
      },
      _ => false
    }
  }

  /// Convert two smaller types into a bigger type.
  ///
  /// # Examples
  ///
  /// ```
  /// use rython::memory::SingleType;
  /// let new_type = VarType::convert_type(
  ///   VarType::Single(SingleType::F64),
  ///   VarType::Single(SingleType::I64));
  ///
  /// assert_eq!(new_type, VarType::Single(SingleType::F64))
  /// ```
  pub fn convert_type(left: &VarType, right: &VarType) -> VarType {
    match left {
      VarType::Single(t0) => {
        match right {
          VarType::Single(t1) => {
            match t0 {
              SingleType::I64 => {
                match t1 {
                  SingleType::I64 => VarType::Single(SingleType::I64),
                  SingleType::F64 => VarType::Single(SingleType::F64),
                  _ => panic!("Cannot convert non-number type yet")
                }
              },
              SingleType::F64 => {
                match t1 {
                  SingleType::I64 => VarType::Single(SingleType::F64),
                  SingleType::F64 => VarType::Single(SingleType::F64),
                  _ => panic!("Cannot convert non-number type yet")
                }
              },
              _ => panic!("Cannot convert non-number type yet")
            }
          },
          _ => panic!("Cannot convert non-number type yet")
        }
      },
      _ => panic!("Cannot convert non-number type yet")
    }
  }
}