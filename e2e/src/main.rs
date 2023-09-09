use ast::*;
use ir::*;
use lexer::*;
use obj::*;
use parser::*;

fn main() {
    let lex = ProseLexer::new("pub const main = fn() { return 5; }");
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.func().unwrap();
    let mut ir = FIRSource::new(0);
    match *ast_parsed {
        Expr::FuncDef(val) => {
            let result = ir.begin(val);
            build_main(result);
        }
        _ => panic!("not a func def!"),
    }
}
