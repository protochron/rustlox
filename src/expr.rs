use crate::tokens::Token;

pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

pub struct BinaryOp {
    pub op_type: BinaryOperationType,
}

pub enum BinaryOperationType {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

pub enum UnaryOperationType {
    Minus,
    Bang,
}
