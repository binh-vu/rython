use pest::iterators::Pair;
use crate::memory::Memory;
use fnv::FnvHashMap;
use crate::compiler::compiler::*;
use crate::compiler::compile_expr::*;
use crate::types::*;
use crate::statements::*;

pub fn pair2return(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Result<Box<dyn Statement>, String> {
  let mut iter = pair.into_inner();

  let ast = ExprASTNode::from_pair(iter.next().unwrap(), mem, name2id);
  let return_type = ast.get_type();

  let name = "return";

  let var_id = match name2id.get(name) {
    None => {
      let id = mem.add_var(&return_type);
      name2id.insert(name.to_string(), VarDef {
        id,
        name: name.to_string(),
        var_type: return_type.clone()
      });

      id
    },
    Some(vardef) => {
      if name2id[name].var_type != return_type {
        return Err(format!("TypeError: previous return statement has type: {:?} while the current return statement has type: {:?}", name2id[name].var_type, return_type));
      }
      vardef.id
    }
  };

  let stmt: Box<dyn Statement> = match &return_type {
    VarType::Single(t) => {
      match t {
        SingleType::Str => {
          Box::new(ReturnStmt {
            var_id,
            expr: ast.compile_str()
          })
        },
        SingleType::I64 => {
          Box::new(ReturnStmt {
            var_id,
            expr: ast.compile_i64()
          })
        }
        SingleType::F64 => {
          Box::new(ReturnStmt {
            var_id,
            expr: ast.compile_f64()
          })
        }
        SingleType::Bool => {
          Box::new(ReturnStmt {
            var_id,
            expr: ast.compile_bool()
          })
        }
      }
    }
    _ => unimplemented!()
  };

  Ok(stmt)
}