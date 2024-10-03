use clap::{arg, Parser, ValueEnum};
use execution::{
    interpreter::Interpreter,
    native::{codegen::CodeGeneration, state::State},
};
use optimize::{peephole::CombineIncrements, OptimizationPass};

pub mod execution;
pub mod optimize;
pub mod syntax;
use std::{io::Write, path::PathBuf};

macro_rules! time {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let result = $e;
        let elapsed = start.elapsed();
        std::io::stdout().flush().unwrap();
        println!("Time: {:?}", elapsed);
        result
    }};
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum, default_value_t=Mode::Compiled)]
    mode: Mode,

    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long)]
    optimize: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Compiled,
    Interpreted,
}

fn main() {
    let cli = Cli::parse();

    let s = std::fs::read_to_string(&cli.path).unwrap();
    let mut nodes = syntax::parse(&s).unwrap();

    if cli.optimize {
        nodes = CombineIncrements.optimize(nodes);
    }

    match cli.mode {
        Mode::Interpreted => {
            time!(Interpreter::new().interpret(&nodes));
        }
        Mode::Compiled => {
            let x = CodeGeneration::x64();
            let code = x.generate(&nodes);
            let mut state = State::new(Box::new(std::io::stdin()), Box::new(std::io::stdout()));
            time!(code.run(&mut state).unwrap());
        }
    }
}
