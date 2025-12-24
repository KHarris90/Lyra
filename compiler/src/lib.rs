pub mod ast;
pub mod driver;
pub mod lexer;
pub mod parser;

use lyra_meta::Diagnostic;

#[derive(Debug)]
pub struct CompileOutput {
    pub module: ast::Module,
    pub diagnostics: Vec<Diagnostic>,
}

#[must_use]
pub fn compile(source: &str) -> CompileOutput {
    let (tokens, mut diags) = lexer::tokenize(source);
    let (module, parse_diags) = parser::parse(&tokens);
    diags.extend(parse_diags);

    CompileOutput {
        module,
        diagnostics: diags,
    }
}
