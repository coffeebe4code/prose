use ir::*;
use lexer::*;
use parser::*;

fn main() {
    let lex = ProseLexer::new("pub const x = fn() { return 5 + 3 * 2; }");
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.func();
    let mut ir = FIRSource::new(0);
    ir.begin(&ast_parsed.unwrap());
}
