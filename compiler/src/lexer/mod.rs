use lyra_meta::{Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String),
    Int(i64),
    Str(String),

    // Minimal punctuation/operators (expand later)
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semi,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[must_use]
pub fn tokenize(source: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    let bytes = source.as_bytes();
    let mut i: usize = 0;

    let mut tokens: Vec<Token> = Vec::new();
    let mut diags: Vec<Diagnostic> = Vec::new();

    while i < bytes.len() {
        skip_ws_and_comments(bytes, &mut i);

        if i >= bytes.len() {
            break;
        }

        let b = bytes[i];

        if is_ident_start(b) {
            tokens.push(lex_ident(source, bytes, &mut i));
            continue;
        }

        if b.is_ascii_digit() {
            match lex_int(source, bytes, &mut i) {
                Ok(tok) => tokens.push(tok),
                Err(d) => diags.push(d),
            }
            continue;
        }

        if b == b'"' {
            match lex_string(source, bytes, &mut i) {
                Ok(tok) => tokens.push(tok),
                Err(d) => {
                    diags.push(d);
                    break; // simplest recovery for now
                }
            }
            continue;
        }

        if let Some(tok) = lex_punct(bytes, &mut i) {
            tokens.push(tok);
            continue;
        }

        diags.push(
            Diagnostic::error(format!("unexpected character: {}", char_for_error(b)))
                .with_span(Span::new(i, i + 1)),
        );
        i += 1;
    }

    (tokens, diags)
}

fn skip_ws_and_comments(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() {
        let b = bytes[*i];

        if b.is_ascii_whitespace() {
            *i += 1;
            continue;
        }

        // Line comment: # ... \n
        if b == b'#' {
            *i += 1;
            while *i < bytes.len() && bytes[*i] != b'\n' {
                *i += 1;
            }
            continue;
        }

        break;
    }
}

fn lex_ident(source: &str, bytes: &[u8], i: &mut usize) -> Token {
    let start = *i;
    *i += 1;
    while *i < bytes.len() && is_ident_continue(bytes[*i]) {
        *i += 1;
    }
    let text = &source[start..*i];
    Token {
        kind: TokenKind::Ident(text.to_string()),
        span: Span::new(start, *i),
    }
}

fn lex_int(source: &str, bytes: &[u8], i: &mut usize) -> Result<Token, Diagnostic> {
    let start = *i;
    *i += 1;
    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
        *i += 1;
    }
    let text = &source[start..*i];
    match text.parse::<i64>() {
        Ok(v) => Ok(Token {
            kind: TokenKind::Int(v),
            span: Span::new(start, *i),
        }),
        Err(_) => Err(
            Diagnostic::error(format!("invalid integer literal: {text}"))
                .with_span(Span::new(start, *i)),
        ),
    }
}

fn lex_string(_source: &str, bytes: &[u8], i: &mut usize) -> Result<Token, Diagnostic> {
    let start = *i;
    *i += 1; // consume opening quote

    let mut out = String::new();
    let mut terminated = false;

    while *i < bytes.len() {
        let c = bytes[*i];

        if c == b'"' {
            *i += 1; // consume closing quote
            terminated = true;
            break;
        }

        if c == b'\\' {
            if *i + 1 >= bytes.len() {
                break;
            }
            let next = bytes[*i + 1];
            match next {
                b'"' => out.push('"'),
                b'\\' => out.push('\\'),
                b'n' => out.push('\n'),
                b't' => out.push('\t'),
                _ => out.push(next as char),
            }
            *i += 2;
            continue;
        }

        out.push(c as char);
        *i += 1;
    }

    if !terminated {
        return Err(Diagnostic::error("unterminated string literal")
            .with_span(Span::new(start, (*i).min(bytes.len()))));
    }

    Ok(Token {
        kind: TokenKind::Str(out),
        span: Span::new(start, *i),
    })
}

fn lex_punct(bytes: &[u8], i: &mut usize) -> Option<Token> {
    let start = *i;
    let b = bytes[*i];

    let kind = match b {
        b'(' => TokenKind::LParen,
        b')' => TokenKind::RParen,
        b'{' => TokenKind::LBrace,
        b'}' => TokenKind::RBrace,
        b',' => TokenKind::Comma,
        b';' => TokenKind::Semi,
        b'+' => TokenKind::Plus,
        b'-' => TokenKind::Minus,
        b'*' => TokenKind::Star,
        b'/' => TokenKind::Slash,
        b'=' => TokenKind::Eq,
        _ => return None,
    };

    *i += 1;
    Some(Token {
        kind,
        span: Span::new(start, *i),
    })
}

fn is_ident_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

fn is_ident_continue(b: u8) -> bool {
    is_ident_start(b) || b.is_ascii_digit()
}

fn char_for_error(b: u8) -> String {
    if b.is_ascii_graphic() || b == b' ' {
        (b as char).to_string()
    } else {
        format!("0x{b:02X}")
    }
}
