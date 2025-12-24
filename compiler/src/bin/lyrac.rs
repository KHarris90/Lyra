use anyhow::Result;
use clap::{Parser, ValueEnum};
use lyra_meta::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lyrac", version = LYRA_VERSION, about = "Lyra compiler driver")]
struct Args {
    /// Path to a .ly file
    file: PathBuf,

    /// What to emit (for debugging)
    #[arg(long, value_enum, default_value_t = EmitArg::None)]
    emit: EmitArg,
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
    println!("{LYRA_NAME} compiler v{LYRA_VERSION}");

    let args = Args::parse();
    let out = compiler::driver::compile_file(&args.file, args.emit.into())?;

    for d in &out.diagnostics {
        compiler::driver::print_diag(d);
    }

    Ok(())
}
