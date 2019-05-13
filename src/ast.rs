#[derive(Debug)]
pub enum Register {
    A, B, C, D
}

#[derive(Debug)]
pub enum Token {
    Num(usize),
    Reg(Register),
    Literal,
    Identifier(String),
    Label,
    Open,
    Close,
    Deref,
    Jump,
    From,
    Inc,
    Halt,
    Backwards,
    Forwards,
    Reverse,
    Io,
    If, Eq,
    Gt, Gte,
    Lt, Lte,
    And, Or,
    Nop,
    Newlines(usize)
}

#[derive(Debug)]
pub enum Dest {
    Reg(Register),
    Addr(usize),
    Deref(Box<Dest>)
}

#[derive(Debug)]
pub enum Source {
    Reg(Register),
    Addr(usize),
    Literal(usize),
    Deref(Box<Source>)
}

#[derive(Debug)]
pub enum Instruction {
    Inc(Dest, Source),
    Jump(Option<String>),
    From(Option<String>),
    Forwards,
    Backwards,
    Reverse,
    Halt,
    Io(Source)
}

#[derive(Debug)]
pub enum Expr {
    Backwards,
    Forwards,
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Gte(Box<Expr>, Box<Expr>),
    Lte(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Lit(Source)
}

#[derive(Debug)]
pub struct Line {
    pub label: Option<String>,
    pub inst: Instruction,
    pub cond: Option<Expr>,
    pub stack: std::vec::Vec<usize>
}
