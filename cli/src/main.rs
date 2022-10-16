use huckleberry_lib::{env::Env, evaluator::eval};
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: huck [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        repl();
    }

    Ok(())
}

fn run_file(path: &str) {
    let contents = std::fs::read_to_string(path).expect(&format!("Unable to read file: {}", path));
    let env = Env::with_core_module().into_ref();
    eval(&contents, env).unwrap();
}

fn repl() {
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
}
