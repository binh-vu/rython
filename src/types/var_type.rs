#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleType {
  Bool,
  Str,
  I64,
  F64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarType {
  Single(SingleType),
  List(SingleType),
  Dict((SingleType, SingleType)),
}

impl VarType {
  pub fn is_number(&self) -> bool {
    match self {
      VarType::Single(t) => {
        match t {
          SingleType::Str => false,
          SingleType::I64 => true,
          SingleType::F64 => true,
          SingleType::Bool => false,
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