use huckleberry_lib::{env::Env, evaluator::eval};
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let env = Env::with_core_module().into_ref();

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match eval(&line, env.clone_ref()) {
                Ok(expr) => {
                    println!("{}", expr);
                    rl.add_history_entry(line);
                }
                Err(err) => {
                    println!("{:?}", err);
                    rl.add_history_entry(line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
