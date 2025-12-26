use compiler::lexer::{tokenize, TokenKind};

#[test]
fn tokenize_appends_eof() {
    let (tokens, diags) = tokenize("let x = 1");
    assert!(diags.is_empty(), "expected no diagnostics: {diags:?}");
    assert!(!tokens.is_empty(), "expected tokens");

    let last = tokens.last().expect("tokens not empty");
    assert!(
        matches!(last.kind, TokenKind::Eof),
        "expected EOF token at end, got: {:?}",
        last.kind
    );
}
