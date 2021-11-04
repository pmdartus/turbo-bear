use super::Location;

#[derive(Debug)]
pub enum LitKind {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub struct Lit {
    pub kind: LitKind,
    pub location: Location,
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct Ty {
    pub name: String,
    pub location: Location,
}

#[derive(Debug)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug)]
pub enum ExprKind {
    Logical(LogicalOp, Box<Expr>, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Ident(Ident),
    Lit(Lit),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub location: Location,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub location: Location,
}

#[derive(Debug)]
pub enum StmtKind {
    Var(Ident, Option<Ty>, Option<Expr>),
    Fn(Ident, Vec<(Ident, Ty)>, Ty, Block),
    Ret(Option<Expr>),
    Expr(Expr),
    Block(Block),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub location: Location,
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
    pub location: Location,
}
