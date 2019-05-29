use std::borrow::Cow;

use fnv::FnvHashMap;
use pest::iterators::Pair;

use crate::memory::Memory;
use crate::statements::statement::Statement;
use crate::types::*;
use crate::expr::arithmetic::*;
use crate::expr::expr::Expr;
use crate::expr::expr_enum::ExprEnum;
use crate::expr::constant_expr::{ConstantExpr, Constant};

use super::Rule;
use super::VarDef;
use std::cmp::max;
use crate::expr::value_expr::VarExpr;


/// Represent an AST of an expression.
///
/// # Examples
///
/// the expression `5 * 2 + 3 * 4` will be represented as:
///
///       +
///    /    \
///   *      *
///  / \    / \
/// 5   2  3   4
///
/// the expression `sqrt(sqrt(5 * 2) + a) * 2` will be represented as
///
///            *
///          /   \
///       sqrt    2
///        |
///        +
///     /    \
///   sqrt    a
///    |
///    *
///  /  \
/// 5    2
pub(super) enum ExprASTNode {
  NegOps(Box<ExprASTNode>),
  Ops((Box<ExprASTNode>, String, Box<ExprASTNode>)),
  Func(Vec<ExprASTNode>),
  Variable(VarDef),
  Constant(Constant),
}


impl ExprASTNode {
  pub fn from_pair(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Box<ExprASTNode> {
    // do a post-order traversal to generate a list of commands
    match pair.as_rule() {
      Rule::expr => {
        let mut iter = pair.into_inner();
        let mut ast = ExprASTNode::from_pair(iter.next().unwrap(), mem, name2id);
        while let Some(add_pair) = iter.next() {
          let mut children = add_pair.into_inner();
          let ops = children.next().unwrap().as_str().to_string();

          ast = Box::new(ExprASTNode::Ops((ast, ops, ExprASTNode::from_pair(children.next().unwrap(), mem, name2id))));
        };

        ast
      }
      Rule::factor => {
        let mut iter = pair.into_inner();
        let mut ast = ExprASTNode::from_pair(iter.next().unwrap(), mem, name2id);
        while let Some(mul_pair) = iter.next() {
          let mut children = mul_pair.into_inner();
          let ops = children.next().unwrap().as_str().to_string();

          ast = Box::new(ExprASTNode::Ops((ast, ops, ExprASTNode::from_pair(children.next().unwrap(), mem, name2id))));
        }

        ast
      }
      Rule::primary => {
        let child = pair.into_inner().next().unwrap();
        match child.as_rule() {
          Rule::expr => {
            ExprASTNode::from_pair(child, mem, name2id)
          }
          Rule::number => {
            Box::new(number2node(child))
          }
          Rule::func_call => {
            Box::new(func2node(child, mem, name2id))
          }
          Rule::var => {
            Box::new(var2node(child, mem, name2id))
          }
          Rule::neg_primary => {
            Box::new(ExprASTNode::NegOps(ExprASTNode::from_pair(child, mem, name2id)))
          }
          Rule::format_string => {
            Box::new(format_string2node(child, mem, name2id))
          }
          Rule::string => {
            Box::new(string2node(child, mem, name2id))
          }
          _ => unreachable!()
        }
      }
      _ => unreachable!()
    }
  }

  pub fn get_type(&self) -> Result<VarType, String> {
    match self {
      ExprASTNode::Constant(c) => {
        match c {
          Constant::I64(_) => Ok(VarType::Single(SingleType::I64)),
          Constant::F64(_) => Ok(VarType::Single(SingleType::F64)),
          Constant::Str(_) => Ok(VarType::Single(SingleType::Str)),
        }
      }
      ExprASTNode::NegOps(c) => {
        let ctype = c.get_type()?;
        if ctype.is_number() {
          Ok(ctype)
        } else {
          Err("Cannot compute negative value from non-number".to_string())
        }
      }
      ExprASTNode::Ops((left, ops, right)) => {
        let ltype = left.get_type()?;
        let rtype = right.get_type()?;

        if !ltype.is_number() || !rtype.is_number() {
          Err("Expect numbers for the expression".to_string())
        } else {
          Ok(VarType::convert_type(&ltype, &rtype))
        }
      }
      ExprASTNode::Func(_) => {
        unimplemented!()
      }
      ExprASTNode::Variable(v) => Ok(v.var_type.clone())
    }
  }

  /// compile the ast into commands, and return the maximum number of slots needed to evaluate
  /// these commands
  ///
  /// # Arguments
  ///
  /// * `registry_no` - the registry slot that is available to write data to
  pub fn compile<T: Type>(self, registry_no: usize, commands: &mut Vec<ExprEnum<T>>) -> usize {
    match self {
      ExprASTNode::Constant(c) => {
        // the expression has only one term that return the value
        commands.push(ExprEnum::Constant(ConstantExpr::new(registry_no, c)));
        registry_no + 1
      },
      ExprASTNode::Variable(v) => {
        commands.push(ExprEnum::Var(VarExpr::new(registry_no, v.id)));
        registry_no + 1
      },
      ExprASTNode::NegOps(c0) => {
        let c = *c0;
        // no matter how many command was executed, the result will be stored back at the registry_no slot
        // so we just need to read the value from there
        let max_registry = c.compile(registry_no, commands);
        commands.push(ExprEnum::Neg(NegExpr::new(registry_no)));
        max_registry
//        match c {
//          ExprASTNode::Constant(_) => {
//            // negative result of a value unit, you should not have this, but this is just in case
//            c.compile(registry_no, commands);
//            commands.push(ExprEnum::Neg(NegExpr::new(registry_no))); // re-use the registry
//            registry_no
//          },
//          ExprASTNode::Ops((left, ops, right)) => {
//            let left_max = left.compile(registry_no, commands);
//            let right_max = right.compile(registry_no + 1, commands);
//
//            match ops.as_str() {
//              "+" => {
//                commands.push(ExprEnum::Add(AddExpr::new(registry_no, registry_no + 1)));
//              },
//              "-" => {
//                commands.push(ExprEnum::Sub(SubExpr::new(registry_no, registry_no + 1)));
//              },
//              "*" => {
//                commands.push(ExprEnum::Mul(MulExpr::new(registry_no, registry_no + 1)));
//              },
//              "/" => {
//                commands.push(ExprEnum::Div(DivExpr::new(registry_no, registry_no + 1)));
//              },
//              _ => unreachable!()
//            }
//
//            commands.push(ExprEnum::Neg(NegExpr::new(registry_no)));
//            max(left_max, right_max)
//          },
//          ExprASTNode::Variable(_) => {
//            c.compile(registry_no, commands);
//            commands.push(ExprEnum::Neg(NegExpr::new(registry_no)));
//            registry_no
//          }
//          _ => unimplemented!()
//        }
      },
      ExprASTNode::Ops((left, ops, right)) => {
        let left_max = left.compile(registry_no, commands);
        let right_max = right.compile(registry_no + 1, commands);

        match ops.as_str() {
          "+" => {
            commands.push(ExprEnum::Add(AddExpr::new(registry_no, registry_no + 1)));
          },
          "-" => {
            commands.push(ExprEnum::Sub(SubExpr::new(registry_no, registry_no + 1)));
          },
          "*" => {
            commands.push(ExprEnum::Mul(MulExpr::new(registry_no, registry_no + 1)));
          },
          "/" => {
            commands.push(ExprEnum::Div(DivExpr::new(registry_no, registry_no + 1)));
          },
          _ => unreachable!()
        }

        max(left_max, right_max)
      },
      _ => unimplemented!()
    }
  }

}
#[inline]
fn number2node(pair: Pair<Rule>) -> ExprASTNode {
  let val = pair.as_span().as_str();
  let constant = match val.find(".") {
    None => Constant::I64(val.parse::<i64>().unwrap()),
    Some(_) => Constant::F64(val.parse::<f64>().unwrap())
  };

  ExprASTNode::Constant(constant)
}

#[inline]
fn string2node(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> ExprASTNode {
  let val = pair.as_str();
  ExprASTNode::Constant(Constant::Str(Cow::Owned(val.to_string())))
}

#[inline]
fn var2node(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> ExprASTNode {
  let name = pair.as_str();
  ExprASTNode::Variable(name2id[name].clone())
}

#[inline]
fn format_string2node(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> ExprASTNode {
  unimplemented!()
}

#[inline]
fn func2node(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> ExprASTNode {
  let mut iter = pair.into_inner();
  let c0 = iter.next().unwrap();
//  let mut func_name;
  unimplemented!()
//  match c0.as_rule() {
//    Rule::identifier => {
//      func_name = c0.as_str().to_string();
//    },
//    _ => {
//      // first is an argument
//
//    }
//  }
}

