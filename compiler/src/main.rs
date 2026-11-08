mod lexer;

fn main() {
    let tests = [
        // Initial table
        "16kb 2mb 1gb 8",
        "0kb",
        "16 kb",
        "16xb",
        "16KB",
        "16k",
        "340282366920938463463374607431768211455kb",
        "1+2kb"
    ];
    
    for src in tests {
        println!("--- {:?}", src);
        match lexer::Lexer::new(src).tokenize() {
            Ok(tokens) => for t in tokens { println!("{:?}", t); },
            Err(e) => println!("error at {}:{}: {}", e.line, e.col, e.message),
        }
    }
}
