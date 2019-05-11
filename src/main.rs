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
    let tokens = self::parse::lex(content);
    let program = self::parse::parse(tokens);
    let labels = match self::eval::scan_labels(&program) {
        eval::ScanResult::Missing(missing) => {
            print!("ERROR: Missing the folowing labels");
            print!("{:?}", missing);
            return;
        },
        eval::ScanResult::Unused(unused, labels) => {
            print!("warning: the folowing labels are unused");
            print!("{:?}", labels);
            labels
        },
        eval::ScanResult::Ok(labels) => labels
    };
    let mut init_ctx = self::eval::Context::new(labels);
    self::eval::eval(program, &mut init_ctx);
}

fn main() {
    moonwalk_main();
}
