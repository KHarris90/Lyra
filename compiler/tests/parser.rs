use compiler::lexer::tokenize;
use compiler::parser::parse;

#[test]
fn parse_let_binding() {
    let src = "let x = 1\n";
    let (tokens, lex_diags) = tokenize(src);
    assert!(lex_diags.is_empty(), "lexer diagnostics: {lex_diags:?}");

    let (m, diags) = parse(&tokens);
    assert!(diags.is_empty(), "parser diagnostics: {diags:?}");
    assert_eq!(m.items.len(), 1);
}

#[test]
fn parse_call_with_args() {
    let src = "print(\"hi\", 1)\n";
    let (tokens, lex_diags) = tokenize(src);
    assert!(lex_diags.is_empty(), "lexer diagnostics: {lex_diags:?}");

    let (m, diags) = parse(&tokens);
    assert!(diags.is_empty(), "parser diagnostics: {diags:?}");
    assert_eq!(m.items.len(), 1);
}

#[test]
fn parse_error_missing_ident_after_let() {
    let src = "let = 1\n";
    let (tokens, _lex_diags) = tokenize(src);
    let (_m, diags) = parse(&tokens);
    assert!(!diags.is_empty(), "expected parser diagnostics");
}

#[test]
fn parse_error_unterminated_call() {
    let src = "print(\n";
    let (tokens, _lex_diags) = tokenize(src);
    let (_m, diags) = parse(&tokens);
    assert!(!diags.is_empty(), "expected parser diagnostics");
}
