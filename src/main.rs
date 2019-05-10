mod bf;
mod ast;

fn main() {
    let lit = self::ast::Source::Literal(4);
    let l = self::ast::Line {
        label: Some("test".to_string()),
        inst: self::ast::Instruction::Inc(
            self::ast::Dest::Deref(Box::new(self::ast::Dest::Reg(self::ast::Register::A))),
            lit
        ),
        cond: None
    };
    self::bf::bfmain();
}
