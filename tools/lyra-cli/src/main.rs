use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use lyra_meta::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lyra", version = LYRA_VERSION, about = "Lyra language CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Compile a Lyra source file
    Compile {
        /// Path to a .ly file
        file: PathBuf,

        /// What to emit (for debugging)
        #[arg(long, value_enum, default_value_t = EmitArg::None)]
        emit: EmitArg,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum EmitArg {
    None,
    Ast,
    Tokens,
}

impl From<EmitArg> for compiler::driver::Emit {
    fn from(v: EmitArg) -> Self {
        match v {
            EmitArg::None => Self::None,
            EmitArg::Ast => Self::Ast,
            EmitArg::Tokens => Self::Tokens,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Compile { file, emit } => {
            let out = compiler::driver::compile_file(&file, emit.into())?;

            for d in &out.diagnostics {
                compiler::driver::print_diag(d);
            }

            println!("compiled {}", file.display());
        }
    }

    Ok(())
}
