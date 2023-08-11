use ir::*;
use lexer::*;
use parser::*;

fn main() {
    let lex = ProseLexer::new("5 + 3 * 2");
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.low_bin();
    let mut ir = IrSource::new();
    panic!();
}
