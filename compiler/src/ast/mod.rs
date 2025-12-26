#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let { name: Ident, expr: Expr },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    Int(i64),
    Str(String),
    Call { callee: Ident, args: Vec<Expr> },
}
