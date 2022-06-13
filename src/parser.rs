use super::token::Token;

pub enum Object {
    S(String),
    N(f64)
}

pub enum Expr {
    Binary {
	left: Box<Expr>,
	operator: Token,
	right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Object),
    Unary {
	operator: Token,
	right: Box<Expr>
    }
}

