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

pub fn deref_source(loc:&ast::Source, ctx: &Context) -> usize{ //IMPLIMENT
    return 0;
    //return ctx.mem[];
}

pub fn get_reg_val(reg: &ast::Register, ctx: &Context) -> usize{
    match reg{
        ast::Register::A => ctx.a,
        ast::Register::B => ctx.b,
        ast::Register::C => ctx.c,
        ast::Register::D => ctx.d,
    }
}

// this is a separate eval for expressions it needs
// to be separate so that an expression can recursively evaualted
pub fn eval_expr(expr: &ast::Expr, ctx: &Context) -> bool {
    match expr{
        ast::Expr::Backwards => !ctx.forward,
        ast::Expr::Forwards => ctx.forward,
        ast::Expr::Or(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::And(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::Gte(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::Lte(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::Gt(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::Lt(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::Eq(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::Lit(val) => {
            match val{
                ast::Source::Reg(reg) => get_reg_val(reg, &ctx)!=0,
                ast::Source::Addr(loc) => ctx.mem[*loc] != 0,
                ast::Source::Literal(val) => *val !=0,
                ast::Source::Deref(loc) => deref_source(loc, ctx) != 0,
            }
        },
    }
}

pub enum ScanResult {
    Missing(Vec<String>),
    Unused(Vec<String>, HashMap<String, usize>),
    Ok(HashMap<String, usize>)
}

// scan a program to get label lookup table
pub fn scan_labels(program: &Vec<ast::Line>) -> ScanResult {
    return ScanResult::Ok(HashMap::new());
}

// evaluate a program
pub fn eval(program: Vec<self::ast::Line>, ctx: &mut Context) {
    loop{
        let mut halted = false;
        let current_line = &program[ctx.pc];




        let end_of_program = (ctx.pc > program.len() && ctx.forward) || (ctx.pc == 0 && !ctx.forward);
        if(halted || end_of_program){
            break;
        }
        if(ctx.forward){
            ctx.pc+=1;
        }
        else{
            ctx.pc-=1;
        }

    }

}
