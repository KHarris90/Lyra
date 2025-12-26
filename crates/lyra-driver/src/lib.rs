use anyhow::{Context, Result};
use lyra_meta::{Diagnostic, Severity};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emit {
    None,
    Ast,
    Tokens,
}

pub fn compile_file(path: &Path, emit: Emit) -> Result<compiler::CompileOutput> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read source file: {}", path.display()))?;

    let out = compiler::compile(&source);

    match emit {
        Emit::None => {}
        Emit::Ast => println!("{:#?}", out.module),
        Emit::Tokens => {
            let (tokens, _diags) = compiler::lexer::tokenize(&source);
            println!("{tokens:#?}");
        }
    }

    Ok(out)
}

pub fn print_diag(d: &Diagnostic) {
    let sev = match d.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    };

    if let Some(span) = d.span {
        eprintln!("{sev}: {} at {}..{}", d.message, span.start, span.end);
    } else {
        eprintln!("{sev}: {}", d.message);
    }
}
