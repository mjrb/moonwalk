use std::io::{self, Read};
use std::env;
use std::fs;

mod bf {
    #[derive(Debug)]
    pub enum Instruction {
        Inc,
        Dec,
        Right,
        Left,
        In,
        Out,
        Nop,
        Loop(Vec<Instruction>)
    }

    pub fn match_loop(input: &String, offset: usize) -> Option<usize> {
        let mut i = match input[offset..].find('[') {
            None => return None,
            Some(num) => num + offset
        };
        let mut count = 0;
        while i < input.len() {
            count += match input.chars().nth(i) {
                None => return None,
                Some(c) => match c {
                    '[' =>  1,
                    ']' => -1,
                    _   =>  0
                }
            };
            if count == 0 {
                return Some(i)
            }
            i += 1;
        }
        return None;
    }

    pub struct Context {
        ptr: usize,
        heap: [u8; 65536]
    }

    pub fn parse(input: String) -> Vec<Instruction> {
        let mut program: Vec<Instruction> = vec![];
        let mut i = 0;
        while i < input.len() {
            program.push(match input.chars().nth(i) {
                Some(c) => match c {
                    '+' => Instruction::Inc,
                    '-' => Instruction::Dec,
                    '>' => Instruction::Right,
                    '<' => Instruction::Left,
                    ',' => Instruction::In,
                    '.' => Instruction::Out,
                    '[' => match match_loop(&input, i) {
                        None => Instruction::Nop,
                        Some(end) => {
                            let begin = i + 1;
                            i = end;
                            Instruction::Loop(parse(input[begin .. end].to_string()))
                        }
                    },
                    _   => Instruction::Nop
                },
                None => Instruction::Nop
            });
            i+=1;
        }
        return program;
    }

    pub fn getc() -> char {
        let mut buf = [0; 1];
        return match io::stdin().read(&mut buf) {
            Ok(_) => buf[0] as char,
            Err(_) => 255 as char
        }
    }

    pub fn eval(program: &Vec<Instruction>, in_ctx: Context) -> Context {
        let mut ctx = in_ctx;
        for op in program {
            //print!("{:?}\n", op);
            match op {
                Instruction::Inc => ctx.heap[ctx.ptr] += 1,
                Instruction::Dec => ctx.heap[ctx.ptr] -= 1,
                Instruction::Right => ctx.ptr += 1,
                Instruction::Left  => ctx.ptr -= 1,
                Instruction::Loop(content) => {
                    while ctx.heap[ctx.ptr] != 0 {
                        ctx = eval(&content, ctx);
                    }
                }
                Instruction::In => ctx.heap[ctx.ptr] = getc() as u8,
                Instruction::Out => print!("{}", ctx.heap[ctx.ptr] as char),
                Instruction::Nop => ()
            }
        }
        return ctx;
    }

    pub fn bfmain() {
        let args: Vec<String> = env::args().collect();
        let message = format!("Unable to read file {}", args[1]);
        let line = fs::read_to_string(&args[1]).expect(&message);
        let program = parse(line);
        let init_ctx = Context {
            ptr: 0,
            heap: [0; 65536]
        };
        eval(&program, init_ctx);
    }
}
