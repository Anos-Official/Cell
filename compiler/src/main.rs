mod lexer;

fn main() {
    let mut lx = lexer::Lexer::new("+ --[[ a\nb\nc ]]-- -\n* /");
    for t in lx.tokenize() {
        println!("{t:?}",)
    }
}
