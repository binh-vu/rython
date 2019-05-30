use pest::iterators::Pair;
use crate::memory::Memory;
use fnv::FnvHashMap;
use crate::compiler::compiler::*;
use crate::statements::{Statement, VarAssignment};
use crate::types::*;
use crate::compiler::compile_expr::*;

pub fn pair2defvar(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Result<Box<dyn Statement>, String> {
  let mut iter = pair.into_inner();
  let name = iter.next().unwrap().as_str();
  let var_type = match iter.next().unwrap().as_str() {
    "str" => VarType::Single(SingleType::Str),
    "int" => VarType::Single(SingleType::I64),
    "float" => VarType::Single(SingleType::F64),
    _ => unimplemented!()
  };
  let expr = iter.next().unwrap();
  let var = VarDef {
    id: mem.add_var(&var_type),
    name: name.to_string(),
    var_type
  };

  let stmt: Box<dyn Statement> = match &var.var_type {
    VarType::Single(t) => {
      match t {
        SingleType::Str => {
          Box::new(VarAssignment::new(var.id, pair2expr_str(expr, mem, name2id, &var.var_type)?))
        },
        SingleType::I64 => {
          Box::new(VarAssignment::new(var.id, pair2expr_i64(expr, mem, name2id, &var.var_type)?))
        },
        SingleType::F64 => {
          Box::new(VarAssignment::new(var.id, pair2expr_f64(expr, mem, name2id, &var.var_type)?))
        },
        SingleType::Bool => {
          Box::new(VarAssignment::new(var.id, pair2expr_bool(expr, mem, name2id, &var.var_type)?))
        }
      }
    }
    _ => unimplemented!()
  };

  name2id.insert(name.to_string(), var);
  Ok(stmt)
}

pub fn pair2assvar(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Result<Box<dyn Statement>, String> {
  let mut iter = pair.into_inner();
  let name = iter.next().unwrap().as_str();
  let expr = iter.next().unwrap();

  let var_id = name2id[name].id;
  let var_type = name2id[name].var_type.clone();
  let stmt: Box<dyn Statement> = match &var_type {
    VarType::Single(t) => {
      match t {
        SingleType::Str => {
          Box::new(VarAssignment::new(var_id, pair2expr_str(expr, mem, name2id, &var_type)?))
        },
        SingleType::I64 => {
          Box::new(VarAssignment::new(var_id, pair2expr_i64(expr, mem, name2id, &var_type)?))
        }
        SingleType::F64 => {
          Box::new(VarAssignment::new(var_id, pair2expr_f64(expr, mem, name2id, &var_type)?))
        }
        SingleType::Bool => {
          Box::new(VarAssignment::new(var_id, pair2expr_bool(expr, mem, name2id, &var_type)?))
        }
      }
    },
    _ => unimplemented!()
  };

  Ok(stmt)
}