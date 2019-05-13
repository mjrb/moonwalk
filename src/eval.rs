use std::collections::HashMap;
use std::str;
use std::io::{self, Read};

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

pub fn getc() -> char {
    let mut buf: [u8; 1] = [0; 1];
    return match io::stdin().read(&mut buf) {
        Ok(_) => buf[0] as char,
        Err(_) => 255 as char
    }
}

//TODO: Check rules on deref
pub fn deref_source(loc:&ast::Source, ctx: &Context) -> usize{
    let location = source_to_val(loc, ctx);
    ctx.mem[location]
}


pub fn deref_dest(loc:&ast::Dest, ctx: &Context) -> usize{
    let location = match loc{
        ast::Dest::Reg(reg) => get_reg_val(reg, ctx),
        ast::Dest::Addr(loc) => ctx.mem[*loc],
        ast::Dest::Deref(dst) => deref_dest(loc, ctx),//TODO deref_dest function
    };
    location
}


pub fn source_to_val(src: &ast::Source, ctx: &Context) -> usize{
    match src{
        ast::Source::Reg(reg) => get_reg_val(reg, ctx),
        ast::Source::Addr(loc) => ctx.mem[*loc],
        ast::Source::Literal(val) => *val,
        ast::Source::Deref(src) => deref_source(src, ctx),//SHOULD I ERROR OR JUST REPEAT?
    }
}

pub fn get_reg_val(reg: &ast::Register, ctx: &Context) -> usize{
    match reg{
        ast::Register::A => ctx.a,
        ast::Register::B => ctx.b,
        ast::Register::C => ctx.c,
        ast::Register::D => ctx.d,
    }
}

pub fn set_reg_val(reg: &ast::Register, val: usize, ctx: &mut Context){
    match reg{
        ast::Register::A => ctx.a = val,
        ast::Register::B => ctx.b = val,
        ast::Register::C => ctx.c = val,
        ast::Register::D => ctx.d = val,
    }
}
// this is a separate eval for expressions it needs
// to be separate so that an expression can recursively evaualted
pub fn eval_expr(expr: &ast::Expr, ctx: &Context) -> bool {
    match expr{
        ast::Expr::Backwards => !ctx.forward,
        ast::Expr::Forwards => ctx.forward,
        ast::Expr::Or(left, right) => eval_expr(left, &ctx) || eval_expr(right,&ctx),
        ast::Expr::And(left, right) => eval_expr(left, &ctx) && eval_expr(right,&ctx),
        ast::Expr::Gte(left, right) => eval_expr(left, &ctx) >= eval_expr(right,&ctx),
        ast::Expr::Lte(left, right) => eval_expr(left, &ctx) <= eval_expr(right,&ctx),
        ast::Expr::Gt(left, right) => eval_expr(left, &ctx) > eval_expr(right,&ctx),
        ast::Expr::Lt(left, right) => eval_expr(left, &ctx) < eval_expr(right,&ctx),
        ast::Expr::Eq(left, right) => eval_expr(left, &ctx) == eval_expr(right,&ctx),
        ast::Expr::Lit(val) => source_to_val(val, ctx) != 0,
    }
}

pub fn jump_to_label(label: &String , ctx: &mut Context){
    match ctx.labels.get(label){
        Some(lineno) => {ctx.pc = *lineno;},
        None =>(),
    }
}

pub enum ScanResult {
    Missing(Vec<String>),
    Unused(Vec<String>, HashMap<String, usize>),
    Duplicate(String),
    Ok(HashMap<String, usize>)
}

// scan a program to get label lookup table
pub fn scan_labels(program: &Vec<ast::Line>) -> ScanResult {
    let mut map = HashMap::new();
    let mut c =0;
    for line in program{
        match &line.label{
            Some(lbl) => {
                let cpy = lbl.clone();
                if(map.contains_key(lbl)){
                    return ScanResult::Duplicate(cpy);
                }
                map.insert(cpy, (c as usize));
            },
            None=>()
        }
        c +=1;
    }
    return ScanResult::Ok(map);
}


pub fn execute_instruction(inst: &ast::Instruction, ctx: &mut Context,) -> (bool, bool, bool){
    match inst{
        ast::Instruction::Inc(dest, src) =>{
            println!("INCREMENT");
            let srcval = source_to_val(src, ctx);
            let destval = match dest{
                ast::Dest::Reg(reg) => get_reg_val(reg, ctx),
                ast::Dest::Addr(loc) => ctx.mem[*loc],
                ast::Dest::Deref(dst) => ctx.mem[deref_dest(dest, ctx)],//TODO deref_dest function
            };
            let mut newval = 0;
            if(ctx.forward){
                newval = destval + srcval;
            }
            else{
                newval = destval - srcval;
            }
            match dest{
                ast::Dest::Reg(reg) => set_reg_val(reg, newval, ctx),
                ast::Dest::Addr(loc) => ctx.mem[*loc] = newval,
                ast::Dest::Deref(dst) => ctx.mem[deref_dest(dest, ctx)] = newval,//TODO deref_dest function
            }

            (false, false, false)
        },
        ast::Instruction::Jump(lbl) =>{
            if(ctx.forward){
                match lbl{
                    Some(label) => {println!("JUMP {}", label); jump_to_label(label,ctx); return (false, true, false)},
                    None => {return (false, false, true)}, //Pop Stack and go
                }
            }
            (false, false, false)
        },
        ast::Instruction::From(lbl) =>{
            if(!ctx.forward){
                match lbl{
                    Some(label) => {jump_to_label(label,ctx); return (false, true, false)},
                    None => {return (false, false, true)}, //Pop Stack and go
                }

            }
            (false, false, false)
        },
        ast::Instruction::Forwards => {
            ctx.forward = true;
            (false, false, false)
        },
        ast::Instruction::Backwards => {
            ctx.forward = false;
            (false, false, false)
        },
        ast::Instruction::Reverse => {
            ctx.forward = !ctx.forward;
            (false, false, false)
        },
        ast::Instruction::Io(src) => {
            if(!ctx.forward){
                let val = source_to_val(src, ctx);
                let val = [val as u8];
                let output = str::from_utf8(&val).expect("not UTF-8");
                println!("{}", output);
            }
            else{
                let mut inp = getc();
                let inp = inp as usize;
                //panic!("Cannot take input");
                match src{
                    ast::Source::Literal(_) => panic!("Cannot place input into a literal"),
                    ast::Source::Reg(reg) => set_reg_val(reg, inp, ctx),
                    ast::Source::Addr(loc) => ctx.mem[*loc] = inp,
                    ast::Source::Deref(src) => ctx.mem[source_to_val(src, ctx)] = inp,
                }
            }
            (false, false, false)
        },
        ast::Instruction::Halt => {
            println!("Program Halted");
            (true, false, false)
        },
    }
}


// evaluate a program
pub fn eval(program: &mut Vec<self::ast::Line>, ctx: &mut Context) {
    let mut program = program;
    loop{
        let currentPC = ctx.pc.clone();
        let mut halted = false;
        let mut jumped = false;
        let mut tojump = false;
        let mut current_line = &program[ctx.pc];
        //Check for option here!!
        match(&current_line.cond){
            Some(cond) => {
                if(eval_expr(cond, &ctx)){
                    let (halted, jumped, tojump)  = execute_instruction(&current_line.inst, ctx,);
                }
            },
            None => {let (halted, jumped, tojump) = execute_instruction(&current_line.inst, ctx,);}, //DO Instruction
        }
        let end_of_program = (ctx.pc > program.len() && ctx.forward) || (ctx.pc == 0 && !ctx.forward);
        if(halted || end_of_program){
            break;
        }
        if(jumped){
            program[ctx.pc].stack.push(currentPC);
        }
        if(tojump){
            let lineno = program[ctx.pc].stack.pop();
            ctx.pc = match lineno{
                None => currentPC,
                Some(newPC) => newPC
            };
        }
        if(ctx.forward){
            ctx.pc+=1;
        }
        else{
            ctx.pc-=1;
        }



    }

}
