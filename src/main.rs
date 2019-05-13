use std::env;
use std::fs;

use std::collections::HashMap;

//mod bf;
mod ast;
mod parse;
mod eval;

fn moonwalk_main() {
    let args: Vec<String> = env::args().collect();
    let message = format!("Unable to read file {}", args[1]);
    let content = fs::read_to_string(&args[1]).expect(&message);
    let tokens = match self::parse::lex(content) {
        Some(toks) => toks,
        None => {
            print!("ERROR: bad token");
            return;
        }
    };
    let mut program = match self::parse::parse(tokens) {
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
            println!("{:?}", labels);
            labels
        },
        eval::ScanResult::Duplicate(dup) => {
            println!("ERROR: Duplicate Label Found:");
            println!("{}", dup);
            return;
        }
        eval::ScanResult::Ok(labels) => labels
    };
    let mut init_ctx = self::eval::Context::new(labels);
    self::eval::eval(&mut program, &mut init_ctx);
}

fn main() {
    moonwalk_main();
    //printInput();
}
