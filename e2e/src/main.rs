use gen::*;
use ir::*;
use lexer::*;
use parser::*;
use vm::*;

fn main() {
    let lex = ProseLexer::new("5 + 3 * 2");
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.low_bin();
    let mut ir = IrSource::new();
    ir.begin_repl(ast_parsed.unwrap().unwrap());
    let mut gen = GenSource::new();
    ir.flush(&mut gen);
    let mut vm = Vm::new(gen);
    let result = vm.run();
    assert_eq!(result.unwrap(), 11);
}
