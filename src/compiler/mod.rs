use fnv::FnvHashMap;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::memory::Memory;
use crate::statements::*;
use crate::types::*;
use crate::expr_tree::*;
use crate::program::Program;
use crate::compiler::compile_expr::ExprASTNode;

pub mod compiler;
pub mod compile_expr;
pub mod compile_if;
pub mod compile_value_assignment;
pub mod compile_pc_control;

pub use self::compiler::RythonCompiler;