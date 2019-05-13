use std::str::FromStr;

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
    let mut tokenizer = Tokenizer::new();
    use crate::ast::Token::{*};

    tokenizer.def_match(r"0x[0-9a-fA-F]+", &|mat: String| {
        Num(usize::from_str_radix(&mat[2..], 16).unwrap())
    });

    tokenizer.def_match(r"[0-9]+", &|mat: String| {
        Num(usize::from_str(&mat).unwrap())
    });

    tokenizer.def_match(
        r"[\$:\(\)\*]|jump|from|inc|halt|io|backwards|forwards|reverse|if|<=|>=|>|=|<|and|or",
        &|mat: String| {
            match mat.as_ref() {
                "$" => Literal,
                ":" => Label,
                "(" => Open,
                ")" => Close,
                "*" => Deref,
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
    // register letter followed by non identifier character
    tokenizer.def_match(r"(?i)A[^a-zA-Z\-0-9]|B[^a-zA-Z\-0-9]|C[^a-zA-Z\-0-9]|D[^a-zA-Z\-0-9]", &|mat: String| {
        match &mat.as_str()[..1] {
            "A" | "a" => Reg(ast::Register::A),
            "B" | "b" => Reg(ast::Register::B),
            "C" | "c" => Reg(ast::Register::C),
            "D" | "d" => Reg(ast::Register::D),
            _ => Nop // imposible
        }
    });
    
    tokenizer.def_match(r"[a-zA-Z\-0-9]+", &|mat: String| {
        Identifier(mat)
    });

    // End of line comment
    tokenizer.def_match(r";.*", &|_: String| {
        Nop
    });

    tokenizer.def_match(r"\n+", &|mat: String| {
        Newlines(mat.len())
    });


    tokenizer.def_match(r"\s*", &|_: String| {
        Nop
    });

    return tokenizer.tokenize(input);
}
