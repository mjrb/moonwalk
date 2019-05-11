mod ast;

pub struct Context {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
    pub mem: [usize; 65536],
    pub forward: bool,
    pub pc: usize
}

pub fn eval(input: Vec<self::ast::Line>, ctx: &mut Context) {
    
}
