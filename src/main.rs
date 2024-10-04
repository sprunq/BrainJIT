use clap::{arg, Parser, ValueEnum};
use execution::{
    interpreter::Interpreter,
    native::{codegen::CodeGeneration, state::State},
};
use optimize::{peephole::*, OptimizationPass};
pub mod execution;
pub mod optimize;
pub mod syntax;
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

macro_rules! time {
    ( $msg:expr, $e:expr) => {{
        let start = std::time::Instant::now();
        let result = $e;
        let elapsed = start.elapsed();
        std::io::stdout().flush().unwrap();
        println!("{}: {:?}", $msg, elapsed);
        result
    }};
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t=Mode::Jit)]
    mode: Mode,

    #[arg(short, long)]
    #[clap(help = "The file to run")]
    path: PathBuf,

    #[arg(short, long)]
    #[clap(help = "Optimize the program")]
    optimize: bool,

    #[arg(short, long)]
    #[clap(help = "Dump the binary to a file. Only works in compiled mode")]
    dump_binary: bool,

    #[arg(short, long, default_value = "30000")]
    #[clap(help = "The number of cells in the tape")]
    tape_size: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Jit,
    Interpret,
}

fn main() {
    let cli = Cli::parse();

    let s = std::fs::read_to_string(&cli.path).unwrap();
    let mut nodes = time!("Parse", syntax::parse(&s).unwrap());

    if cli.optimize {
        nodes = time!(
            "OptimizeCombineIncrements",
            CombineIncrements.optimize(nodes)
        );
        nodes = time!("OptimizeReplaceSet", ReplaceSet.optimize(nodes));
        nodes = time!("OptimizeCombineSets", CombineSets.optimize(nodes));
    }

    if cli.dump_binary {
        let mut file = std::fs::File::create("optimized.txt").unwrap();
        writeln!(file, "{}", syntax::indented(&nodes, 0)).unwrap();
    }

    match cli.mode {
        Mode::Interpret => {
            time!(
                "Execution finished in",
                Interpreter::new(
                    Box::new(std::io::stdin()),
                    Box::new(std::io::stdout()),
                    cli.tape_size
                )
                .interpret(&nodes)
            );
        }
        Mode::Jit => {
            if std::env::consts::ARCH != "x86_64" {
                panic!("Only x86_64 is supported");
            }

            let codegen = CodeGeneration::x86_x64();
            let executor = codegen.generate(&nodes);

            if cli.dump_binary {
                executor.dump_binary("out.bin");
            }

            let mut state = State::new(
                Box::new(std::io::stdin()),
                Box::new(std::io::stdout()),
                cli.tape_size,
            );
            let result = time!("Execution finished in", executor.run(&mut state));
            if result.is_error() {
                eprintln!("Error: {:?}", result);
            }
        }
    }
}
