use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use crate::{
    environment::Environment,
    interpreter::eval,
};

mod environment;
mod error;
mod expr;
mod interpreter;
mod modules;
mod parser;
mod scanner;


fn main() -> Result<()> {
    let env = Environment::with_core_module().into_ref();

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match eval(&line, env.clone_ref()) {
                    Ok(expr) => {
                        println!("{}", expr);
                        rl.add_history_entry(line);
                    }
                    Err(err) => println!("{:?}", err)
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    Ok(())
}