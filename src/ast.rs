#[derive(Clone, Debug)]
pub enum Statement {
    ExprStatement(Box<Expr>),
    Let(String, String, Box<Expr>),
    Print(Box<Expr>)
}
#[derive(Clone, Debug)]
pub enum Expr {
    Number(f64),
    Id(String),
    PI,
    E,
    Parenthesis(Box<Expr>),
    Exponent(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Eq(String, Box<Expr>),
}