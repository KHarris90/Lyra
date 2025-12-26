use lyra_meta::prelude::*;

use crate::ast::{Expr, Ident, Item, Module, Stmt};
use crate::lexer::{Token, TokenKind};

#[must_use]
pub fn parse(tokens: &[Token]) -> (Module, Vec<Diagnostic>) {
    let mut p = Parser::new(tokens);
    let module = p.parse_module();
    (module, p.diags)
}

struct Parser<'a> {
    tokens: &'a [Token],
    i: usize,
    diags: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            i: 0,
            diags: Vec::new(),
        }
    }

    fn parse_module(&mut self) -> Module {
        let mut items = Vec::new();

        while !self.is_eof() {
            if let Some(stmt) = self.parse_stmt() {
                items.push(Item::Stmt(stmt));
            } else {
                // basic recovery: consume one token to avoid infinite loop
                self.bump();
            }
        }

        Module { items }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        if self.at_ident_kw("let") {
            self.parse_let_stmt()
        } else {
            self.parse_expr_stmt()
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.bump(); // 'let' (as Ident("let"))

        let name = if let Some(s) = self.bump_ident_value() {
            Ident { name: s }
        } else {
            self.error_here("expected identifier after 'let'");
            return None;
        };

        if !self.eat_symbol_eq() {
            self.error_here("expected '=' in let binding");
            return None;
        }

        let Some(expr) = self.parse_expr() else {
            self.error_here("expected expression after '='");
            return None;
        };

        Some(Stmt::Let { name, expr })
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr()?;
        Some(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_call_or_primary()
    }

    fn parse_call_or_primary(&mut self) -> Option<Expr> {
        let primary = self.parse_primary()?;

        // Only identifiers can be callees in milestone 1
        if let Expr::Ident(callee) = primary {
            if self.at_symbol_lparen() {
                return self.parse_call(callee);
            }
            return Some(Expr::Ident(callee));
        }

        Some(primary)
    }

    fn parse_call(&mut self, callee: Ident) -> Option<Expr> {
        if !self.eat_symbol_lparen() {
            self.error_here("expected '(' after function name");
            return None;
        }

        let mut args = Vec::new();

        if self.at_symbol_rparen() {
            self.bump();
            return Some(Expr::Call { callee, args });
        }

        loop {
            let Some(arg) = self.parse_expr() else {
                self.error_here("expected expression in argument list");
                return None;
            };
            args.push(arg);

            if self.eat_symbol_comma() {
                continue;
            }

            if self.at_symbol_rparen() {
                self.bump();
                break;
            }

            self.error_here("expected ',' or ')' after argument");
            self.sync_to_rparen_or_eof();
            if self.at_symbol_rparen() {
                self.bump();
            }
            break;
        }

        Some(Expr::Call { callee, args })
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek_kind() {
            Some(TokenKind::Ident(_)) => {
                let s = self.bump_ident_value()?;
                Some(Expr::Ident(Ident { name: s }))
            }
            Some(TokenKind::Int(_)) => {
                let n = self.bump_int_value()?;
                Some(Expr::Int(n))
            }
            Some(TokenKind::Str(_)) => {
                let s = self.bump_str_value()?;
                Some(Expr::Str(s))
            }
            _ => None,
        }
    }

    // ----------------------------
    // token helpers (match your lexer)
    // ----------------------------

    fn is_eof(&self) -> bool {
        self.i >= self.tokens.len()
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.i)
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    fn bump(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.i);
        if t.is_some() {
            self.i += 1;
        }
        t
    }

    fn at_ident_kw(&self, kw: &str) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Ident(s)) if s == kw)
    }

    fn bump_ident_value(&mut self) -> Option<String> {
        match self.peek_kind() {
            Some(TokenKind::Ident(_)) => {
                let t = self.bump()?;
                if let TokenKind::Ident(s) = &t.kind {
                    Some(s.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn bump_int_value(&mut self) -> Option<i64> {
        let t = self.bump()?;
        if let TokenKind::Int(n) = t.kind {
            Some(n)
        } else {
            None
        }
    }

    fn bump_str_value(&mut self) -> Option<String> {
        let t = self.bump()?;
        if let TokenKind::Str(s) = &t.kind {
            Some(s.clone())
        } else {
            None
        }
    }

    // ----------------------------
    // punctuation helpers
    // NOTE: If your variants are named differently, rename them here only.
    // ----------------------------

    fn at_symbol_lparen(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::LParen))
    }

    fn at_symbol_rparen(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::RParen))
    }

    fn eat_symbol_lparen(&mut self) -> bool {
        if self.at_symbol_lparen() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_symbol_comma(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Comma)) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_symbol_eq(&mut self) -> bool {
        if matches!(self.peek_kind(), Some(TokenKind::Eq)) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn sync_to_rparen_or_eof(&mut self) {
        while !self.is_eof() && !self.at_symbol_rparen() {
            self.bump();
        }
    }

    fn error_here(&mut self, msg: &'static str) {
        let span = self.peek().map_or_else(|| Span::new(0, 0), |t| t.span);

        self.diags.push(Diagnostic {
            severity: Severity::Error,
            message: msg.to_string(),
            span: Some(span),
        });
    }
}
