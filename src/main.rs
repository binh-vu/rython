use pest::Parser;
use rython::compiler::*;
use std::time::Instant;
use std::fs::{File, read_to_string};
use rython::expr_tree::*;
use fnv::FnvHashMap;

fn main() -> Result<(), String> {
  let code = read_to_string("/workspace/rython/tests/resources/program_a.ry").unwrap();


  let mut program = RythonCompiler::compile(&code)?;
//  println!("{:#?}", program.statements);
  let start = Instant::now();
  program.exec();
//  println!("[main] {:?}", program.exec());
  println!(">>> [main] runtime: {:?}", start.elapsed());

  Ok(())
}
