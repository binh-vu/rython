use std::borrow::Cow;
use fnv::FnvHashMap;
use crate::types::*;

#[derive(Default, Debug, Clone)]
pub struct Memory {
  pub str_vars: Vec<Str>,
  pub i64_vars: Vec<i64>,
  pub f64_vars: Vec<f64>,

  pub list_str_vars: Vec<Vec<Str>>,
  pub list_i64_vars: Vec<Vec<i64>>,
  pub list_f64_vars: Vec<Vec<f64>>,

  pub dict_str_str_vars: Vec<FnvHashMap<Str, Str>>,
  pub dict_str_i64_vars: Vec<FnvHashMap<Str, i64>>,
  pub dict_str_f64_vars: Vec<FnvHashMap<Str, f64>>,

  pub dict_int_str_vars: Vec<FnvHashMap<i64, Str>>,
  pub dict_int_i64_vars: Vec<FnvHashMap<i64, i64>>,
  pub dict_int_f64_vars: Vec<FnvHashMap<i64, f64>>,
}

impl Memory {
  pub fn add_var(&mut self, var_type: &VarType) -> usize {
    match var_type {
      VarType::Single(s) => {
        match s {
          SingleType::Str => {
            self.str_vars.push(Cow::Owned(String::new()));
            self.str_vars.len() - 1
          }
          SingleType::I64 => {
            self.i64_vars.push(0);
            self.i64_vars.len() - 1
          }
          SingleType::F64 => {
            self.f64_vars.push(0.0);
            self.f64_vars.len() - 1
          }
        }
      },
      _ => panic!("Encounter bug when adding variable")
    }
  }

  #[inline]
  pub fn get_str(&self, var_id: usize) -> Str {
    return self.str_vars[var_id].clone();
  }

  #[inline]
  pub fn set_str(&mut self, var_id: usize, val: Str) {
    self.str_vars[var_id] = val;
  }

  #[inline]
  pub fn get_i64(&self, var_id: usize) -> i64 {
    return self.i64_vars[var_id];
  }

  #[inline]
  pub fn set_i64(&mut self, var_id: usize, val: i64) {
    self.i64_vars[var_id] = val;
  }

  #[inline]
  pub fn get_f64(&self, var_id: usize) -> f64 {
    return self.f64_vars[var_id];
  }

  #[inline]
  pub fn set_f64(&mut self, var_id: usize, val: f64) {
    self.f64_vars[var_id] = val;
  }
}
