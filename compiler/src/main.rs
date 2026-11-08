mod lexer;

fn main() {
    let tests = [
        // Initial table
        "",
        "  \n ",
        "+ - * /",
        "= == < <= > >= !=",
        "===",
        "<<=>",
        "0 007 255",
        "0.5 1.25 100.0",
        "1..2",
        "5-3",
        "5--3",
        
        // B. Lines and Windows line endings
        "1\n  2\n\n3",
        "1\r\n2",
        
        // C. Comments
        "1 -- c\n2",
        "--",
        "--[ x\n1",
        "1 --[[ a\nb ]]-- 2",
        "--[[ ]]--",
        
        // D. Things that should produce errors, not panics
        "@",
        "1 +\n  $",
        "\t@",
        "é",
        "~",
        "!",
        "1 != !",
        "340282366920938463463374607431768211456",
        "1 340282366920938463463374607431768211456",
        "+ --[[ never closed",
        "--[[",
        "--[[ ]]-",
    ];
    
    for src in tests {
        println!("--- {:?}", src);
        match lexer::Lexer::new(src).tokenize() {
            Ok(tokens) => for t in tokens { println!("{:?}", t); },
            Err(e) => println!("error at {}:{}: {}", e.line, e.col, e.message),
        }
    }
}
