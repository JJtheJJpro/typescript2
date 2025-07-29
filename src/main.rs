use crate::grammar::TS2G;

mod grammar;

fn main() {
    let mut parser = TS2G::init();
    parser.parse("let x: u64 = 1+1; print(x); x = x + 10; print(x);").unwrap();
}
