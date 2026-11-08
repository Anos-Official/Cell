mod lexer;

fn main() {
    let mut lx = lexer::Lexer::new("2.5 10 3.14159 7.");
    for t in lx.tokenize() {
        println!("{t:?}",)
    }
    println!("2nd");
    let mut lx = lexer::Lexer::new("1.2.3");
    for t in lx.tokenize() {
        println!("{t:?}",)
    }
}
