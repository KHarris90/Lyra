use lyra_meta::prelude::Diagnostic;

use crate::ast::Module;
use crate::lexer::Token;

#[must_use]
pub fn parse(_tokens: &[Token]) -> (Module, Vec<Diagnostic>) {
    // TODO: real parser
    (Module::new(), Vec::new())
}
