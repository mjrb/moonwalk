use std::collections::HashMap;

use crate::ast;

pub struct Context {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
    pub mem: [usize; 65536],
    pub forward: bool,
    pub pc: usize,
    pub labels: HashMap<String, usize>
}
impl Context {
    pub fn new(labels: HashMap<String, usize>) -> Context {
        return Context {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            mem: [0; 65536],
            forward: true,
            pc: 0,
            labels: labels
        }
    }
}

// this is a separate eval for expressions it needs
// to be separate so that an expression can recursively evaualted
pub fn eval_expr(expr: ast::Expr, ctx: &Context) -> bool {
    return false;
}

// scan a program to get label lookup table
pub fn scan_labels(program: &Vec<ast::Line>) -> HashMap<String, usize> {
    return HashMap::new();
}

// evaluate a program
pub fn eval(program: Vec<self::ast::Line>, ctx: &mut Context) {
    
}
