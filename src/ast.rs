#[derive(Debug)]
pub enum Register {
    A, B, C, D
}

pub enum Token {
    Num(usize),
    Reg(Register),
    Literal,
    Identifier(String),
    Label,
    Open,
    Close,
    Jump,
    From,
    Inc,
    Halt,
    Backwards,
    Forwards,
    Reverse,
    If,
    Gt, Gte,
    Lt, Lte
}

pub enum Dest {
    Reg(Register),
    Addr(usize),
    Deref(Box<Dest>)
}

pub enum Source {
    Reg(Register),
    Addr(usize),
    Literal(usize),
    Deref(Box<Source>)
}

pub enum Instruction {
    Inc(Dest, Source),
    Jump(Option<String>),
    From(Option<String>),
    Halt
}

pub enum Expr {
    Backwards,
    Forwards,
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    
}

pub struct Line {
    pub label: Option<String>,
    pub inst: Instruction,
    pub cond: Option<Expr>
}