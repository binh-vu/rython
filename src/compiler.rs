use fnv::FnvHashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

use crate::memory::Memory;
use crate::statements::declare_stmt::DefVar;
use crate::statements::statement::Statement;
use crate::types::*;
use crate::expr::expr::Expr;
use crate::compiler::compile_expr::ExprASTNode;
use crate::program::Program;

pub mod compile_expr;

#[derive(Parser)]
#[grammar = "rython.pest"]
pub struct RythonCompiler;

#[derive(Debug, Clone)]
pub struct VarDef {
  pub name: String,
  pub id: usize,
  pub var_type: VarType,
}

impl RythonCompiler {
  pub fn compile(program: &str) -> Result<Program, String> {
    let pairs = match RythonCompiler::parse(Rule::program, program) {
      Result::Err(e) => {
        return Err(format!("{}", e));
      },
      Result::Ok(x) => x
    };

    let mut name2id: FnvHashMap<String, VarDef> = FnvHashMap::default();
    let mut mem = Memory::default();
    let mut stmts = vec![];

    for pair in pairs {
      match pair.as_rule() {
        Rule::def_var_stmt => {
          stmts.push(RythonCompiler::pair2vardef(pair, &mut mem, &mut name2id))
        },
        _ => {
          println!("[compile] {:?}", pair.as_rule());
          println!("[compile] {:?}", pair);
        }
      }
    }

    Ok(Program {
      memory: mem,
      statements: stmts
    })
  }

  fn pair2vardef(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Statement {
    let mut iter = pair.into_inner();
    let name = iter.next().unwrap().as_span().as_str();
    let vtype = iter.next().unwrap().as_span().as_str();
    let vexpr = iter.next().unwrap();

    let var_type = match vtype {
      "str" => VarType::Single(SingleType::Str),
      "int" => VarType::Single(SingleType::I64),
      "float" => VarType::Single(SingleType::F64),
      _ => unimplemented!()
    };

    let var = VarDef {
      id: mem.add_var(&var_type),
      name: name.to_string(),
      var_type
    };

    let stmt = match &var.var_type {
      VarType::Single(t) => {
        match t {
          SingleType::Str => {
            Statement::DefVarStr(DefVar::new(var.id, RythonCompiler::pair2expr(vexpr, mem, name2id)))
          },
          SingleType::I64 => {
            Statement::DefVarI64(DefVar::new(var.id, RythonCompiler::pair2expr(vexpr, mem, name2id)))
          }
          SingleType::F64 => {
            Statement::DefVarF64(DefVar::new(var.id, RythonCompiler::pair2expr(vexpr, mem, name2id)))
          }
        }
      }
      _ => unimplemented!()
    };

    name2id.insert(name.to_string(), var);
    stmt
  }

  fn pair2expr<T: Type>(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Expr<T> {
    let ast = ExprASTNode::from_pair(pair, mem, name2id);
    let expr_type = ast.get_type();

    let mut cmds = vec![];
    let n_registry = ast.compile(0, &mut cmds);

    Expr {
      registry: vec![Default::default(); n_registry],
      commands: cmds
    }
//    match expr_type {
//      VarType::Single(t) => {
//        match t {
//          SingleType::Str => {
//            ast.compile()
//          },
//          SingleType::I64 => {}
//          SingleType::F64 => {}
//        }
//      }
//      VarType::List(_) => unimplemented!(),
//      VarType::Dict(_) => unimplemented!()
//    }
  }
}