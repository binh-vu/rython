use fnv::FnvHashMap;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::memory::Memory;
use crate::statements::*;
use crate::types::*;
use crate::expr_tree::*;
use crate::program::Program;
use crate::compiler::compile_expr::{ExprASTNode, pair2expr_bool};
use super::compile_value_assignment::*;
use crate::compiler::compile_pc_control::pair2return;

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
      }
      Result::Ok(x) => x
    };

    let mut name2id: FnvHashMap<String, VarDef> = FnvHashMap::default();
    let mut mem = Memory::default();
    let mut stmts: Vec<Box<dyn Statement>> = vec![];

    for pair in pairs {
      RythonCompiler::compile_pair(pair, &mut mem, &mut name2id, &mut stmts, 0);
    }

    // do a final pass to edit correct pc for return statements
    for i in 0..stmts.len() {
      match stmts[i].get_type() {
        StatementType::Placeholder(0) => {
          stmts[i] = Box::new(Goto {
            pc: stmts.len()
          })
        },
        _ => {}
      }
    }

    Ok(Program {
      memory: mem,
      return_var_id: name2id["return"].id,
      return_type: name2id["return"].var_type.clone(),
      statements: stmts,
    })
  }

  fn compile_pair(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, stmts: &mut Vec<Box<dyn Statement>>, code_lvl: usize) -> Result<(), String> {
    match pair.as_rule() {
      Rule::def_var_stmt => {
        stmts.push(pair2defvar(pair, mem, name2id)?);
      }
      Rule::assign_stmt => {
        stmts.push(pair2assvar(pair, mem, name2id)?);
      }
      Rule::return_stmt => {
        stmts.push(pair2return(pair, mem, name2id)?);
        stmts.push(Box::new(Placeholder::new(0)))
      }
      Rule::if_stmt => {
        RythonCompiler::compile_if(pair, mem, name2id, stmts, code_lvl)?
      }
      Rule::for_stmt => {
        unimplemented!()
      }
      Rule::break_stmt => {
        unimplemented!()
      }
      Rule::continue_stmt => {
        unimplemented!()
      }
      _ => {
        println!("[compile] {:?}", pair.as_rule());
        println!("[compile] {:?}", pair);
      }
    }

    Ok(())
  }

  fn compile_for(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, stmts: &mut Vec<Box<dyn Statement>>, code_lvl: usize) -> Result<(), String> {
    unimplemented!()
  }

  /// Parse an if statements below into a sequence of statements.
  ///
  /// if condition {
  ///   statement* (n_statements)
  /// } else {
  ///   statement* (m_statements)
  /// }
  ///
  /// Compile to:
  ///
  /// i:          <if with false pc = i+n+1>
  /// i+1:           <true_stmt_1>
  /// i+2:           <true_stmt_2>
  ///                ...
  /// i+n:           <true_stmt_n>
  /// i+n+1:      <finish pc = i+n+m+2>
  /// i+n+2:         <false_stmt_1>
  /// i+n+m+1:       <false_stmt_m>
  /// i+n+m+2:    <next_statement>
  fn compile_if(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, stmts: &mut Vec<Box<dyn Statement>>, code_lvl: usize) -> Result<(), String> {
    let mut iter = pair.into_inner();
    let condition_expr = iter.next().unwrap();
    let true_branch = iter.next().unwrap();
    let false_branch = iter.next().unwrap();

    let if_pc_idx = stmts.len();

    stmts.push(Box::new(Placeholder::new(code_lvl))); // assign if pc
    for pair in true_branch.into_inner() {
      RythonCompiler::compile_pair(pair, mem, name2id, stmts, code_lvl + 1);
    }

    let finish_pc_idx = stmts.len();

    stmts.push(Box::new(Placeholder::new(code_lvl))); // assign finish pc
    for pair in false_branch.into_inner() {
      RythonCompiler::compile_pair(pair, mem, name2id, stmts, code_lvl + 1);
    }

    stmts[if_pc_idx] = Box::new(IfFalseGoto {
      condition: pair2expr_bool(condition_expr, mem, name2id, &VarType::Single(SingleType::Bool))?,
      pc: finish_pc_idx,
    });
    stmts[finish_pc_idx] = Box::new(Goto {
      pc: stmts.len()
    });
    Ok(())
  }
}