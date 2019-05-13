use std::env;
use std::fs;

use std::collections::HashMap;

//mod bf;
mod ast;
mod parse;
mod lex;
mod eval;

fn moonwalk_main() {
    let args: Vec<String> = env::args().collect();
    let message = format!("Unable to read file {}", args[1]);
    let content = fs::read_to_string(&args[1]).expect(&message);
    let tokens = match self::lex::lex(content) {
        Some(toks) => toks,
        None => {
            print!("ERROR: bad token");
            return;
        }
    };
    let program = match self::parse::parse(tokens) {
        std::result::Result::Ok(prg) => prg,
        std::result::Result::Err((lineno, e)) => {
            println!("PARSE ERROR ON LINE {}: {}", lineno, e);
            return;
        }
    };
    let labels = match self::eval::scan_labels(&program) {
        eval::ScanResult::Missing(missing) => {
            println!("ERROR: Missing the folowing labels");
            println!("{:?}", missing);
            return;
        },
        eval::ScanResult::Unused(unused, labels) => {
            println!("warning: the folowing labels are unused");
            println!("{:?}", unused);
            labels
        },
        eval::ScanResult::Ok(labels) => labels
    };
    let mut init_ctx = self::eval::Context::new(labels);
    self::eval::eval(program, &mut init_ctx);
}

fn main() {
    moonwalk_main();
    //infinite10();
    //printInput();
}

fn infiniteA(){
    let line1 = ast::Line{
        label: None,
        inst: ast::Instruction::Forwards,
        cond: None,
    };
    let line2 = ast::Line{
        label: None,
        inst: ast::Instruction::Io(ast::Source::Literal(65)),
        cond: Some(ast::Expr::Forwards),
    };
    let line3 = ast::Line{
        label: None,
        inst: ast::Instruction::Backwards,
        cond: None,
    };
    let line4 = ast::Line{
        label: None,
        inst: ast::Instruction::Halt,
        cond: None,
    };
    let mut vec = Vec::new();
    vec.push(line1);
    vec.push(line2);
    vec.push(line3);
    vec.push(line4);
    let mut init_ctx = self::eval::Context::new(HashMap::new());
    self::eval::eval(vec, &mut init_ctx);
}

fn printInput(){

    let line1 = ast::Line{
        label: None,
        inst: ast::Instruction::Forwards,
        cond: None,
    };
    let line2 = ast::Line{
        label: None,
        inst: ast::Instruction::Io(ast::Source::Reg(ast::Register::A)),
        cond: None,//Some(ast::Expr::Forwards),
    };
    let line3 = ast::Line{
        label: None,
        inst: ast::Instruction::Backwards,
        cond: None,
    };
    let line4 = ast::Line{
        label: None,
        inst: ast::Instruction::Halt,
        cond: None,
    };
    let mut vec = Vec::new();
    vec.push(line1);
    vec.push(line2);
    vec.push(line3);
    vec.push(line4);
    let mut init_ctx = self::eval::Context::new(HashMap::new());
    self::eval::eval(vec, &mut init_ctx);

}
