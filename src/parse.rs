use std::result::Result;
use std::collections::VecDeque;

use crate::ast;

pub fn parse_src(tokens: &mut VecDeque<ast::Token>) -> Result<ast::Source, &'static str> {
    use crate::ast::Token::{*};
    let mut derefs = 0;
    while !tokens.is_empty() {
        match tokens.front().unwrap() {
            Deref => {
                derefs += 1;
                tokens.pop_front();
            },
            _ => break // parens are done
        }
    }

    let mut src = match tokens.pop_front().unwrap() {
        Reg(r) => ast::Source::Reg(r),
        Num(n) => ast::Source::Addr(n),
        Literal => match tokens.pop_front() {
            None => return Err("unexpected end of program"),
            Some(tok) => match tok {
                Num(n) => ast::Source::Literal(n),
                _ => return Err("expected number after $")
            }
        }
        t => return Err("invalid source or destination")
    };

    while derefs > 0 {
        src = ast::Source::Deref(Box::new(src));
        derefs -= 1;
    }
    return Ok(src);
}

pub fn src2dest(src: ast::Source) -> Result<ast::Dest, &'static str> {
    match src {
        ast::Source::Literal(_) => Err("Destination Can't Be Literal"),
        ast::Source::Reg(r) => Ok(ast::Dest::Reg(r)),
        ast::Source::Addr(a) => Ok(ast::Dest::Addr(a)),
        ast::Source::Deref(b) => src2dest(*b).map(|dest| ast::Dest::Deref(Box::new(dest)))
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
            Io => parse_src(&mut q).map(|src| {
                ast::Instruction::Io(src)
            }),
            _ => Err("Not an Instruction")
        }
    }
}

pub enum ExprType {
    None,
    Terminal,
    NonTerminal
}

pub fn expr_type(tok: &ast::Token) -> ExprType {
    use crate::ast::Token::{*};
    match tok {
        Reg(_) => ExprType::Terminal,
        Backwards => ExprType::Terminal,
        Forwards => ExprType::Terminal,
        Literal => ExprType::Terminal,
        Num(_) => ExprType::Terminal,
        Close => ExprType::Terminal,
        Deref => ExprType::Terminal,
        Open => ExprType::NonTerminal,
        Or | And => ExprType::NonTerminal,
        Gt | Gte | Eq | Lte | Lt => ExprType::NonTerminal,
        _ => ExprType::None
    }
}

pub fn precedence(a: &ast::Token) -> usize {
    use crate::ast::Token::{*};
    match a {
        Open => 0,
        Gte | Gt | Eq | Lt | Lte => 3,
        And => 2,
        Or => 1,
        _ => 5,
    }
}
pub fn op_map(tok: ast::Token, a: ast::Expr, b: ast::Expr) -> Option<ast::Expr> {
    use crate::ast::Token::{*};
    let ba = Box::new(a);
    let bb = Box::new(b);

    Some(match tok {
        Or => ast::Expr::Or(ba, bb),
        And => ast::Expr::And(ba, bb),
        Gte => ast::Expr::Gte(ba, bb),
        Gt => ast::Expr::Gt(ba, bb),
        Eq => ast::Expr::Eq(ba, bb),
        Lt => ast::Expr::Lt(ba, bb),
        Lte => ast::Expr::Lte(ba, bb),
        _ => return None
    })
}

pub fn op_pop(operators: &mut Vec<ast::Token>, operands: &mut Vec<ast::Expr>) ->
    Result<(), &'static str>
{
    let top = match operators.pop() {
        Some(op) => op,
        None => return Err("mismatched Parentheses")
    };
    let b = match operands.pop() {
        Some(expr) => expr,
        None => return Err("malformed expression, not enough operands")
    };
    let a = match operands.pop() {
        Some(expr) => expr,
        None => return Err("malformed expression, not enough operands")
    };
    return Ok(operands.push(match op_map(top, a, b) {
        Some(res) => res,
        None => return Err("malformed expression, Invalid operator")
    }));
}

pub fn parse_expr(mut q: &mut VecDeque<ast::Token>) -> Result<ast::Expr, &'static str> {
    use crate::ast::Token::{*};
    let mut operands: Vec<ast::Expr> = vec![];
    let mut operators: Vec<ast::Token> = vec![];

    // and now for the tricky bit
    while !q.is_empty() {
        match expr_type(q.front().unwrap()) {
            ExprType::None => {break;},
            ExprType::Terminal => {
                match q.front().unwrap() {
                    Backwards => {
                        q.pop_front();
                        operands.push(ast::Expr::Backwards);
                    },
                    Forwards => {
                        q.pop_front();
                        operands.push(ast::Expr::Forwards);
                    },
                    Close => {
                        q.pop_front();
                        loop {
                            match operators[operators.len() - 1] {
                                Open => {
                                    operators.pop();
                                    break;
                                },
                                _ => match op_pop(&mut operators, &mut operands){
                                    Ok(_) => continue,
                                    Err(e) => return Err(e)
                                }
                            };
                        };
                    }
                    _ => match parse_src(&mut q) {
                        Ok(src) => operands.push(ast::Expr::Lit(src)),
                        Err(e) => return Err(e)
                    }
                };
            }
            ExprType::NonTerminal => {
                match q.front().unwrap() {
                    Open => operators.push(q.pop_front().unwrap()),
                    _ => {
                        while operators.len() > 0 {
                            let top = &operators[operators.len() - 1];
                            if precedence(top) < precedence(q.front().unwrap()) {
                                break;
                            }
                            match op_pop(&mut operators, &mut operands){
                                Ok(res) => res,
                                Err(e) => return Err(e)
                            };
                        }
                        operators.push(q.pop_front().unwrap());
                    }
                }
            }
        }
    }
    // final eval
    while operators.len() > 0 {
        match op_pop(&mut operators, &mut operands){
            Ok(res) => res,
            Err(e) => return Err(e)
        };
    }
    if operands.len() != 1 {
        return Err("malformed expression")
    }
    return Ok(operands.pop().unwrap());
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
            _ => {
                println!("{:?}",q);
                Err("Expected if condition or newline")
            }
        }
    }
}

pub fn parse(tokens: Vec<ast::Token>) -> Result<Vec<ast::Line>, (usize, &'static str)> {
    use crate::ast::Token::{*};
    let mut q = VecDeque::from(tokens);
    let mut lines = vec![];
    let mut lineno = 1;

    'start: while !q.is_empty() {
        let mut label: Option<String> = None;
        // read label
        match q.pop_front().unwrap() {
            Nop => continue,
            Newlines(n) => {
                lineno += n;
                match label {
                    Some(_) => return Err((lineno, "no newline after label")),
                    None => continue
                }
            },
            Identifier(ident) => match q.pop_front() {
                Some(Label) => {
                    match label {
                        Some(_) => return Err((lineno, "already labeled")),
                        None => label = Some(ident.to_string())
                    }
                },
                _ => return Err((lineno, "malformed label"))
            },
            tok => q.push_front(tok) // successful label or no label
        }

        // read instruction
        lines.push(ast::Line{
            label,
            inst: match parse_inst(&mut q) {
                Ok(inst) => inst,
                Err(e) => return Err((lineno, e))
            },
            cond: match parse_cond(&mut q) {
                Ok(cond) => cond,
                Err(e) => return Err((lineno, e))
            }
        });

    }
    return Ok(lines);
}
