mod lexer;

fn main() {
    let mut lx = lexer::Lexer::new("= == < <= > >= !=");
    for t in lx.tokenize() {
        println!("{t:?}",)
    }
}
