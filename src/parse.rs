use std::str::FromStr;
use std::result::Result;
use std::collections::VecDeque;

extern crate regex;
use regex::Regex;

use crate::ast;

trait MatchHandler {
    fn handle(&self, mat: String) -> ast::Token;
}

impl<F> MatchHandler for F where F: Fn(String) -> ast::Token {
    fn handle(&self, mat: String) -> ast::Token {self(mat)}
}

struct TokenMatcher<'l> {
    regex: Regex,
    on_match: &'l MatchHandler
}

impl<'l> TokenMatcher<'l> {
    pub fn try_match(&mut self, iter: &String, tokens: &mut Vec<ast::Token>) -> Option<String>{
        match self.regex.captures(&iter) {
            Some(captures) => {
                if captures[1].to_string() == "".to_string() {
                    return None
                }
                tokens.push(self.on_match.handle(captures[1].to_string()));
                Some(captures[2].to_string())
            }
            None => None
        }
    }
}

struct Tokenizer<'l> {
    matchers: Vec<TokenMatcher<'l>>
}

impl<'l> Tokenizer<'l> {
    pub fn new() -> Tokenizer<'l> {
        return Tokenizer{matchers: Vec::new()};
    }
    pub fn def_match<F>(&mut self, sregex: &str, on_match: &'l F) ->
        std::result::Result<(), regex::Error>
        where F: MatchHandler
    {
        let formatted = &format!(r"^[ ]*({})(?s)(.*)$", sregex.to_string());
        Regex::new(formatted).map(|regex| {
            self.matchers.push(TokenMatcher{regex, on_match})
        })
    }

    pub fn tokenize(&mut self, mut input: String) -> Option<Vec<ast::Token>> {
        let mut tokens: Vec<ast::Token> = vec![];
        'start_matching: while input.len() > 0 {
            let mut i = 0;
            while i < self.matchers.len() {
                match self.matchers[i].try_match(&input, &mut tokens) {
                    Some(new_input) => {
                        input = new_input;
                        continue 'start_matching;
                    },
                    None => ()
                };
                i+=1;
            }
            // no match
            return None
        }
        return Some(tokens);
    }
}

pub fn lex(input: String) -> Option<Vec<ast::Token>> {
    let mut result: Vec<ast::Token> = vec![];
    let mut tokenizer = Tokenizer::new();
    use crate::ast::Token::{*};

    tokenizer.def_match(r"0x[0-9a-fA-F]+", &|mat: String| {
        Num(usize::from_str_radix(&mat[2..], 16).unwrap())
    });

    tokenizer.def_match(r"[0-9]+", &|mat: String| {
        Num(usize::from_str(&mat).unwrap())
    });

    tokenizer.def_match(
        r"[\$:\(\)]|jump|from|inc|halt|io|backwards|forwards|reverse|if|<=|>=|>|=|<|and|or",
        &|mat: String| {
            match mat.as_ref() {
                "$" => Literal,
                ":" => Label,
                "(" => Open,
                ")" => Close,
                "jump" => Jump,
                "from" => From,
                "inc" => Inc,
                "halt" => Halt,
                "io" => Io,
                "backwards" => Backwards,
                "forwards" => Forwards,
                "reverse" => Reverse,
                "if" => If,
                ">=" => Gte,
                "<=" => Lte,
                ">" => Gt,
                "=" => Eq,
                "<" => Lt,
                "and" => And,
                "or" => Or,
                _ => Nop // wont happen, guarded by regex
            }
        }
    );

    tokenizer.def_match(r"[a-zA-Z\-0-9]+", &|mat: String| {
        Identifier(mat)
    });

    // End of line comment
    tokenizer.def_match(r";.*", &|mat: String| {
        Nop
    });

    tokenizer.def_match(r"\n+", &|mat: String| {
        Newlines(mat.len())
    });

    tokenizer.def_match(r"\s*", &|mat: String| {
        Nop
    });

    return tokenizer.tokenize(input);
}

pub fn parse_reg(input: &str) -> Option<ast::Register>{
    use crate::ast::Register::{*};
    match input {
        "A" => Some(A),
        "B" => Some(B),
        "C" => Some(C),
        "D" => Some(D),
        _ => None,
    }

}

pub fn parse_src(tokens: &mut VecDeque<ast::Token>) -> Result<ast::Source, &'static str> {
    use crate::ast::Token::{*};
    let mut opens = 0;
    let mut src = ast::Source::Addr(0);
    while !tokens.is_empty() {
        src = match tokens.pop_front().unwrap() {
            Open => {
                opens += 1;
                continue;
            },
            Identifier(ident) => match parse_reg(&ident) {
                Some(r) => ast::Source::Reg(r),
                None => return Err("invalid register")
            },
            Num(n) => ast::Source::Addr(n),
            Literal => match tokens.pop_front() {
                None => return Err("unexpected end of program"),
                Some(tok) => match tok {
                    Num(n) => ast::Source::Literal(n),
                    _ => return Err("expected number after $")
                }
            }
            tok => return Err("invalid source or destination")
        };
        break;
    }
    let mut closes = 0;
    while !tokens.is_empty() && closes < opens {
        match tokens.pop_front().unwrap() {
            Close => {
                closes += 1;
                src = ast::Source::Deref(Box::new(src));
            },
            _ => return Err("invalid destination")
        }
    }
    if closes < opens {
        return Err("missmatched parentheses");
    }
    return Ok(src);
}

pub fn src2dest(src: ast::Source) -> Result<ast::Dest, &'static str> {
    match src {
        ast::Source::Literal(_) => Err("Destination Can't Be Literal"),
        ast::Source::Reg(r) => Ok(ast::Dest::Reg(r)),
        ast::Source::Addr(a) => Ok(ast::Dest::Addr(a)),
        ast::Source::Deref(b) => src2dest(*b)
    }
}

pub fn pop_if_ident(q: &mut VecDeque<ast::Token>) -> Option<String> {
    // if there is no label immediately after jump token, it
    // becomes bare jump
    match q.front_mut() {
        Some(tok) => match tok {
            ast::Token::Identifier(ident) => {
                let res = Some(ident.clone());
                q.pop_front();
                res
            },
            _ => None
        },
        None => None
    }
}

pub fn parse_inst(mut q: &mut VecDeque<ast::Token>) -> Result<ast::Instruction, &'static str> {
    use crate::ast::Token::{*};
    match q.pop_front() {
        None => Err("unexpected end of program"),
        Some(tok) => match tok {
            Halt => Ok(ast::Instruction::Halt),
            Backwards => Ok(ast::Instruction::Backwards),
            Forwards => Ok(ast::Instruction::Forwards),
            Reverse => Ok(ast::Instruction::Reverse),
            Jump => Ok(ast::Instruction::Jump(pop_if_ident(&mut q))),
            From => Ok(ast::Instruction::From(pop_if_ident(&mut q))),
            Inc => {
                parse_src(&mut q)
                .and_then(|src| src2dest(src))
                .and_then(|dest| {
                    parse_src(&mut q).map(|src| {
                        ast::Instruction::Inc(dest, src)
                    })
                })
            },
            Io => parse_src(&mut q).map(|src| ast::Instruction::Io(src)),
            _ => Err("Not an Instruction")
        }
    }
}

pub fn parse_expr(q: &mut VecDeque<ast::Token>) -> Result<ast::Expr, &'static str> {
    return Ok(ast::Expr::Backwards);
}

pub fn parse_cond(mut q: &mut VecDeque<ast::Token>) -> Result<Option<ast::Expr>, &'static str> {
    use crate::ast::Token::{*};
    // clear writespace or comments
    while !q.is_empty() {
        match q.front_mut().unwrap() {
            Nop => {q.pop_front();}
            _ => {break;}
        }
    }
    match q.front_mut() {
        None => Ok(None),
        Some(tok) => match tok {
            If => {
                q.pop_front();
                parse_expr(&mut q).map(|expr| Some(expr))
            }
            Newlines(_) => Ok(None),
            _ => Err("Expected if condition or newline")
        }
    }
}

pub fn parse(mut tokens: Vec<ast::Token>) -> Result<Vec<ast::Line>, &'static str> {
    use crate::ast::Token::{*};
    let mut q = VecDeque::from(tokens);
    let mut lines = vec![];
    let mut lineno = 0;
    let mut src: Option<ast::Source> = None;
    'start: while !q.is_empty() {
        let mut label: Option<String> = None;
        // read label
        match q.pop_front().unwrap() {
            Nop => continue,
            Newlines(n) => {
                lineno += n;
                match label {
                    Some(_) => return Err("no newline after label"),
                    None => {
                        label = None;
                        continue;
                    }
                }
            },
            Identifier(ident) => match q.pop_front() {
                Some(Label) => {
                    match label {
                        Some(_) => return Err("already labeled"),
                        None => label = Some(ident.to_string())
                    }
                },
                _ => return Err("malformed label")
            },
            tok => q.push_front(tok) // successful label or no label
        }

        // read instruction
        lines.push(ast::Line{
            label,
            inst: match parse_inst(&mut q) {
                Ok(inst) => inst,
                Err(e) => return Err(e)
            },
            cond: match parse_cond(&mut q) {
                Ok(cond) => cond,
                Err(e) => return Err(e)
            }
        });

    }
    return Ok(lines);
}
