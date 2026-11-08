mod lexer;

fn main() {
    let tests = [
        r#""""#,                    // 1
        r#""dead""#,                // 2
        r#""a b" + 1"#,             // 3
        r#""-- not a comment""#,    // 4
        r#""a\nb""#,                // 5
        r#""say \"hi\"""#,          // 6
        r#""tab\there""#,           // 7
        r#""back\\slash""#,         // 8
        r#""a" "b""#,               // 9
        r#""a"+"b""#,               // 10
        "\"a\"\n+",                 // 11 (real line break)
        r#""héllo é""#,             // 12
        r#""\\""#,                  // 13
        r#""abc"#,                  // 14
        "\"abc\ndef\"",             // 15 (real line break)
        r#"1 + "x\q""#,             // 16
        r#""abc\"#,                 // 17
        r#""abc\""#,                // 18
        r#"""#,                     // 19
        r#""\q""#,                  // 20
        "\"ab\\\nc\"",              // 21 (backslash, then a real line break)
        r#""ok" "bad"#,             // 22
    ];
    
    for src in tests {
        println!("--- {:?}", src);
        match lexer::Lexer::new(src).tokenize() {
            Ok(tokens) => for t in tokens { println!("{:?}", t); },
            Err(e) => println!("error at {}:{}: {}", e.line, e.col, e.message),
        }
    }
}
