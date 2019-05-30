use std::borrow::Cow;
use std::cmp::max;
use std::marker::PhantomData;

use fnv::FnvHashMap;
use pest::iterators::Pair;

use crate::expr_tree::*;
use crate::memory::Memory;
use crate::types::*;

use super::compiler::*;

pub fn pair2expr_i64(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, expected_type: &VarType) -> Result<Box<dyn Eval<i64>>, String> {
  let ast = ExprASTNode::from_pair(pair, mem, name2id);
  let ast_type = ast.get_type();
  if ast_type == *expected_type {
    Ok(ast.compile_i64())
  } else {
    Err(format!("TypeError: The expression is expected to have type: `{:?}`, but get type `{:?}`", expected_type, ast_type))
  }
}

pub fn pair2expr_f64(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, expected_type: &VarType) -> Result<Box<dyn Eval<f64>>, String> {
  let ast = ExprASTNode::from_pair(pair, mem, name2id);
  let ast_type = ast.get_type();
  if ast_type == *expected_type {
    Ok(ast.compile_f64())
  } else {
    Err(format!("TypeError: The expression is expected to have type: `{:?}`, but get type `{:?}`", expected_type, ast_type))
  }
}

pub fn pair2expr_str(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, expected_type: &VarType) -> Result<Box<dyn Eval<Str>>, String> {
  let ast = ExprASTNode::from_pair(pair, mem, name2id);
  let ast_type = ast.get_type();
  if ast_type == *expected_type {
    Ok(ast.compile_str())
  } else {
    Err(format!("TypeError: The expression is expected to have type: `{:?}`, but get type `{:?}`", expected_type, ast_type))
  }
}

pub fn pair2expr_bool(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>, expected_type: &VarType) -> Result<Box<dyn Eval<bool>>, String> {
  let ast = ExprASTNode::from_pair(pair, mem, name2id);
  let ast_type = ast.get_type();
  if ast_type == *expected_type {
    Ok(ast.compile_bool())
  } else {
    Err(format!("TypeError: The expression is expected to have type: `{:?}`, but get type `{:?}`", expected_type, ast_type))
  }
}


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
#[derive(Debug)]
pub enum ExprASTNode {
  UnaryOp(UnaryOpNode),
  BinaryOp(BinaryOpNode),
  Func(FuncNode),
  Variable(VarDef),
  Constant(Value),
}

#[derive(Debug)]
pub struct UnaryOpNode {
  node_type: VarType,
  op: String,
  lhs: Box<ExprASTNode>,
}

#[derive(Debug)]
pub struct BinaryOpNode {
  node_type: VarType,
  lhs: Box<ExprASTNode>,
  op: String,
  rhs: Box<ExprASTNode>,
}

#[derive(Debug)]
pub struct FuncNode {
  node_type: VarType,
  func_name: String,
  args: Vec<(VarType, Box<ExprASTNode>)>,
}

impl ExprASTNode {
  pub fn from_pair(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> Box<ExprASTNode> {
    // do a post-order traversal to generate a list of commands
    match pair.as_rule() {
      Rule::expr => {
        let mut iter = pair.into_inner();
        let mut lhs = ExprASTNode::from_pair(iter.next().unwrap(), mem, name2id);
        while let Some(add_pair) = iter.next() {
          let mut children = add_pair.into_inner();
          let op = children.next().unwrap().as_str().to_string();
          let rhs = ExprASTNode::from_pair(children.next().unwrap(), mem, name2id);

          lhs = Box::new(ExprASTNode::BinaryOp(BinaryOpNode {
            node_type: VarType::convert_type(&lhs.get_type(), &rhs.get_type()),
            lhs,
            op,
            rhs,
          }));
        };

        lhs
      }
      Rule::factor => {
        let mut iter = pair.into_inner();
        let mut lhs = ExprASTNode::from_pair(iter.next().unwrap(), mem, name2id);
        while let Some(mul_pair) = iter.next() {
          let mut children = mul_pair.into_inner();
          let op = children.next().unwrap().as_str().to_string();
          let rhs = ExprASTNode::from_pair(children.next().unwrap(), mem, name2id);

          lhs = Box::new(ExprASTNode::BinaryOp(BinaryOpNode {
            node_type: VarType::convert_type(&lhs.get_type(), &rhs.get_type()),
            lhs,
            op,
            rhs,
          }));
        }

        lhs
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
            let lhs = ExprASTNode::from_pair(child.into_inner().next().unwrap(), mem, name2id);
            Box::new(ExprASTNode::UnaryOp(UnaryOpNode {
              node_type: lhs.get_type(),
              op: "-".to_string(),
              lhs,
            }))
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
      _ => unreachable!(pair)
    }
  }

  /// Get type of the current node
  pub fn get_type(&self) -> VarType {
    match self {
      ExprASTNode::Constant(c) => {
        match c {
          Value::I64(_) => VarType::Single(SingleType::I64),
          Value::F64(_) => VarType::Single(SingleType::F64),
          Value::Str(_) => VarType::Single(SingleType::Str),
          Value::Bool(_) => VarType::Single(SingleType::Bool)
        }
      }
      ExprASTNode::UnaryOp(n) => n.node_type.clone(),
      ExprASTNode::BinaryOp(n) => n.node_type.clone(),
      ExprASTNode::Func(n) => n.node_type.clone(),
      ExprASTNode::Variable(n) => n.var_type.clone()
    }
  }

  /// compile the ast that is evaluated to an i64 value
  pub fn compile_bool(self) -> Box<dyn Eval<bool>> {
    match self {
      ExprASTNode::Constant(c) => {
        Box::new(ConstantExpr {
          val: c.as_bool()
        })
      }
      ExprASTNode::Variable(v) => {
        Box::new(VarExpr {
          var_id: v.id
        })
      }
      ExprASTNode::BinaryOp(binary_op) => {
        match binary_op.op.as_str() {
          "+" => {
            Box::new(AddExpr::new(binary_op.lhs.compile_bool(), binary_op.rhs.compile_bool()))
          }
          "-" => {
            Box::new(SubExpr::new(binary_op.lhs.compile_bool(), binary_op.rhs.compile_bool()))
          }
          "*" => {
            Box::new(MulExpr::new(binary_op.lhs.compile_bool(), binary_op.rhs.compile_bool()))
          }
          "/" => {
            Box::new(DivExpr::new(binary_op.lhs.compile_bool(), binary_op.rhs.compile_bool()))
          }
          _ => unimplemented!()
        }
      }
      ExprASTNode::UnaryOp(unary_op) => {
        match unary_op.op.as_str() {
          "-" => {
            panic!("Cannot invoke neg on bool type")
          }
          _ => unreachable!()
        }
      }
      ExprASTNode::Func(func) => {
        let mut args = vec![];
        for (arg_type, arg) in func.args.into_iter() {
          match arg_type {
            VarType::Single(t) => {
              match t {
                SingleType::I64 => {
                  args.push(WrappedEval::I64(arg.compile_i64()));
                }
                SingleType::Str => {
                  args.push(WrappedEval::Str(arg.compile_str()));
                }
                SingleType::F64 => {
                  args.push(WrappedEval::F64(arg.compile_f64()));
                }
                SingleType::Bool => {
                  args.push(WrappedEval::Bool(arg.compile_bool()));
                }
              }
            }
            _ => unimplemented!()
          }
        }

        Box::new(FuncExpr {
          name: func.func_name.clone(),
          args,
        })
      }
    }
  }

  pub fn compile_i64(self) -> Box<dyn Eval<i64>> {
    match self {
      ExprASTNode::Constant(c) => {
        Box::new(ConstantExpr {
          val: c.as_i64()
        })
      }
      ExprASTNode::Variable(v) => {
        Box::new(VarExpr {
          var_id: v.id
        })
      }
      ExprASTNode::BinaryOp(binary_op) => {
        match binary_op.op.as_str() {
          "+" => {
            Box::new(AddExpr::new(binary_op.lhs.compile_i64(), binary_op.rhs.compile_i64()))
          }
          "-" => {
            Box::new(SubExpr::new(binary_op.lhs.compile_i64(), binary_op.rhs.compile_i64()))
          }
          "*" => {
            Box::new(MulExpr::new(binary_op.lhs.compile_i64(), binary_op.rhs.compile_i64()))
          }
          "/" => {
            Box::new(DivExpr::new(binary_op.lhs.compile_i64(), binary_op.rhs.compile_i64()))
          }
          _ => unimplemented!()
        }
      }
      ExprASTNode::UnaryOp(unary_op) => {
        match unary_op.op.as_str() {
          "-" => {
            Box::new(NegExpr::new(unary_op.lhs.compile_i64()))
          }
          _ => unreachable!()
        }
      }
      ExprASTNode::Func(func) => {
        let mut args = vec![];
        for (arg_type, arg) in func.args.into_iter() {
          match arg_type {
            VarType::Single(t) => {
              match t {
                SingleType::I64 => {
                  args.push(WrappedEval::I64(arg.compile_i64()));
                }
                SingleType::Str => {
                  args.push(WrappedEval::Str(arg.compile_str()));
                }
                SingleType::F64 => {
                  args.push(WrappedEval::F64(arg.compile_f64()));
                }
                SingleType::Bool => {
                  args.push(WrappedEval::Bool(arg.compile_bool()));
                }
              }
            }
            _ => unimplemented!()
          }
        }

        Box::new(FuncExpr {
          name: func.func_name.clone(),
          args,
        })
      }
    }
  }

  pub fn compile_f64(self) -> Box<dyn Eval<f64>> {
    match self {
      ExprASTNode::Constant(c) => {
        Box::new(ConstantExpr {
          val: c.as_f64()
        })
      }
      ExprASTNode::Variable(v) => {
        Box::new(VarExpr {
          var_id: v.id
        })
      }
      ExprASTNode::BinaryOp(binary_op) => {
        match binary_op.op.as_str() {
          "+" => {
            Box::new(AddExpr::new(binary_op.lhs.compile_f64(), binary_op.rhs.compile_f64()))
          }
          "-" => {
            Box::new(SubExpr::new(binary_op.lhs.compile_f64(), binary_op.rhs.compile_f64()))
          }
          "*" => {
            Box::new(MulExpr::new(binary_op.lhs.compile_f64(), binary_op.rhs.compile_f64()))
          }
          "/" => {
            Box::new(DivExpr::new(binary_op.lhs.compile_f64(), binary_op.rhs.compile_f64()))
          }
          _ => unimplemented!()
        }
      }
      ExprASTNode::UnaryOp(unary_op) => {
        match unary_op.op.as_str() {
          "-" => {
            Box::new(NegExpr::new(unary_op.lhs.compile_f64()))
          }
          _ => unreachable!()
        }
      }
      ExprASTNode::Func(func) => {
        let mut args = vec![];
        for (arg_type, arg) in func.args.into_iter() {
          match arg_type {
            VarType::Single(t) => {
              match t {
                SingleType::I64 => {
                  args.push(WrappedEval::I64(arg.compile_i64()));
                }
                SingleType::Str => {
                  args.push(WrappedEval::Str(arg.compile_str()));
                }
                SingleType::F64 => {
                  args.push(WrappedEval::F64(arg.compile_f64()));
                }
                SingleType::Bool => {
                  args.push(WrappedEval::Bool(arg.compile_bool()));
                }
              }
            }
            _ => unimplemented!()
          }
        }

        Box::new(FuncExpr {
          name: func.func_name.clone(),
          args,
        })
      }
    }
  }

  pub fn compile_str(self) -> Box<dyn Eval<Str>> {
    match self {
      ExprASTNode::Constant(c) => {
        Box::new(ConstantExpr {
          val: c.as_str()
        })
      }
      ExprASTNode::Variable(v) => {
        Box::new(VarExpr {
          var_id: v.id
        })
      }
      ExprASTNode::BinaryOp(binary_op) => {
        match binary_op.op.as_str() {
          "+" => {
            Box::new(AddExpr::new(binary_op.lhs.compile_str(), binary_op.rhs.compile_str()))
          }
          _ => panic!("TypeError: cannot perform `{}` operator on a string type", binary_op.op)
        }
      }
      ExprASTNode::UnaryOp(unary_op) => {
        match unary_op.op.as_str() {
          "-" => {
            panic!("TypeError: cannot perform `-` operator on a string type")
          }
          _ => unreachable!()
        }
      }
      ExprASTNode::Func(func) => {
        let mut args = vec![];
        for (arg_type, arg) in func.args.into_iter() {
          match arg_type {
            VarType::Single(t) => {
              match t {
                SingleType::I64 => {
                  args.push(WrappedEval::I64(arg.compile_i64()));
                }
                SingleType::Str => {
                  args.push(WrappedEval::Str(arg.compile_str()));
                }
                SingleType::F64 => {
                  args.push(WrappedEval::F64(arg.compile_f64()));
                }
                SingleType::Bool => {
                  args.push(WrappedEval::Bool(arg.compile_bool()));
                }
              }
            }
            _ => unimplemented!()
          }
        }

        Box::new(FuncExpr {
          name: func.func_name.clone(),
          args,
        })
      }
    }
  }
}

#[inline]
fn number2node(pair: Pair<Rule>) -> ExprASTNode {
  let val = pair.as_span().as_str();
  let constant = match val.find(".") {
    None => Value::I64(val.parse::<i64>().unwrap()),
    Some(_) => Value::F64(val.parse::<f64>().unwrap())
  };

  ExprASTNode::Constant(constant)
}

#[inline]
fn string2node(pair: Pair<Rule>, mem: &mut Memory, name2id: &mut FnvHashMap<String, VarDef>) -> ExprASTNode {
  let val = pair.as_str();
  ExprASTNode::Constant(Value::Str(Cow::Owned(val.to_string())))
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

