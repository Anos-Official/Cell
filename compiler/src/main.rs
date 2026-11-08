mod lexer;

fn main() {
    let mut lx = lexer::Lexer::new("255 340282366920938463463374607431768211455 007");
    for t in lx.tokenize() {
        println!("{t:?}",)
    }
}
