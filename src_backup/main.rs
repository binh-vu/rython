use rython::compiler::*;
use std::time::Instant;
use rython::statements::statement::Statement;
use std::fs::{File, read_to_string};
use rython::expr::Eval;

fn main() -> Result<(), String> {
  let code = read_to_string("/workspace/rython/tests/resources/program_a.ry").unwrap();

  let start = Instant::now();
  let mut program = RythonCompiler::compile(&code)?;

  match program.statements.last_mut().unwrap() {
    Statement::DefVarI64(defvar) => {
      println!("[main] {:?}", defvar);
      println!("[main] {:#?}", defvar.expr);
      println!("[main] result = {}", defvar.expr.eval(&program.memory));
    }
    _ => {}
  }

//  println!("{:#?}", program.statements);
  println!(">>> [main] runtime: {:?}", start.elapsed());

  Ok(())
}
