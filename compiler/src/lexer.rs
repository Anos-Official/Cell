//! The lexer turns source text into tokens.

//[[
// Enums
// ]]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Eof,

    Equals,
    EqualsEquals,
    NotEquals,
    LesserThan,
    LesserThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,

    Int(u128),
    Float(f64),
    Dot,
    Size(u128),

    Text(String),
}

//[[
// Structs
// ]]

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    pub line: usize,
    pub col: usize,
}

impl Lexer {
    ///Creates a new lexer;
    /// Takes in a Vector containing Characters
    /// The lexer pointer position, line and col
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    
    ///Returns current character under the Lexer pointer
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    ///Returns next character at the provided offset infont of the lexers pointer
    pub fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    ///Moves the Lexer Pointer forward and updates the current line and col
    pub fn advance(&mut self) -> Option<char> {
        let char_check = self.peek();

        match char_check {
            Some(c) => {
                self.pos += 1;

                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }

                Some(c)
            }
            None => None,
        }
    }

    ///Tokenizes the code
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                
                '+' => {
                    let line = self.line;
                    let col = self.col;
                    self.advance();

                    tokens.push(Token {
                        kind: TokenKind::Plus,
                        line: line,
                        col: col,
                    });
                }

                '-' => {
                    //Is it a comment? if not its a minus

                    let line = self.line;
                    let col = self.col;
                    
                    if self.peek_at(1) == Some('-') {
                        //Is it a multiline comment?
                        if self.peek_at(2) == Some('[') && self.peek_at(3) == Some('[') {
                            //Consume --[[
                            for _ in 0..4 {
                                self.advance();
                            }

                            //Is this the end of multiline?, if not keep going
                            loop {
                                let current = self.peek();

                                if !current.is_some() {
                                    return Err(LexError { message: String::from("Unterminated Comment, did you forget to add ']]--'?"),
                                        line: line, col: col });
                                } else if self.peek() == Some(']')
                                    && self.peek_at(1) == Some(']')
                                    && self.peek_at(2) == Some('-')
                                    && self.peek_at(3) == Some('-') {
                                    break;
                                }

                                self.advance();
                            }

                            self.advance();
                            
                            //Consume ]]--
                            for _ in 0..4 {
                                self.advance();
                            }
                        } else {
                            //Its not multiline, loop until newline or nothing
                            loop {
                                match self.advance() {
                                    Some(c) => {
                                        if c == '\n' {
                                            break;
                                        }
                                    }

                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                    } else {
                        //Its not a comment, its a minus sign
                        self.advance();

                        tokens.push(Token {
                            kind: TokenKind::Minus,
                            line: line,
                            col: col,
                        });
                    }
                }

                '*' => {
                    let line = self.line;
                    let col = self.col;
                    self.advance();

                    tokens.push(Token {
                        kind: TokenKind::Star,
                        line: line,
                        col: col,
                    });
                }

                '/' => {
                    let line = self.line;
                    let col = self.col;
                    self.advance();

                    tokens.push(Token {
                        kind: TokenKind::Slash,
                        line: line,
                        col: col,
                    });
                }

                '=' => {
                    let mut token = TokenKind::Equals;

                    let line = self.line;
                    let col = self.col;
                    self.advance();
                    
                    //Is it EqEq(Comparison)
                    if self.peek() == Some('=') {
                        self.advance();
                        token = TokenKind::EqualsEquals;
                    }

                    tokens.push(Token {
                        kind: token,
                        line: line,
                        col: col,
                    });
                }

                '<' => {
                    let mut token = TokenKind::LesserThan;

                    
                    let line = self.line;
                    let col = self.col;
                    self.advance();
                    
                    //Is it LessEq
                    if self.peek() == Some('=') {
                        self.advance();
                        token = TokenKind::LesserThanOrEquals;
                    }


                    tokens.push(Token {
                        kind: token,
                        line: line,
                        col: col,
                    });
                }

                '>' => {
                    let mut token = TokenKind::GreaterThan;

                    let line = self.line;
                    let col = self.col;
                    self.advance();
                    
                    //Is it EqEq(Comparison)
                    if self.peek() == Some('=') {
                        self.advance();
                        token = TokenKind::GreaterThanOrEquals;
                    }

                    tokens.push(Token {
                        kind: token,
                        line: line,
                        col: col,
                    });
                }

                '!' => {
                    if self.peek_at(1) == Some('=') {
                        let line = self.line;
                        let col = self.col;
                        self.advance();
                        self.advance();
    
                        tokens.push(Token {
                            kind: TokenKind::NotEquals,
                            line: line,
                            col: col,
                        });
                        
                    } else {
                        return Err(LexError { 
                            message: String::from("Unexpected '!', did you mean '!='?"),
                            line: self.line,
                            col: self.col });
                    }
                }

                '0'..='9' => {
                    let line = self.line;
                    let col = self.col;

                    let mut digits = String::new();
                    let mut is_decimal = false;
                    let mut is_size = false;
                    let mut multiplier = 0;
                    
                    while let Some(d) = self.peek() {
                        if d.is_ascii_digit() {
                            digits.push(d);
                            self.advance();
                        } else if d == '.'
                            && !is_decimal
                            && matches!(self.peek_at(1), Some(n) if n.is_ascii_digit())
                        {
                            is_decimal = true;
                            digits.push('.');
                            self.advance();
                        } else if d.is_ascii_alphabetic() {
                            let mut suffix = String::from(d);
                            self.advance();

                            while let Some(ch) = self.peek() {
                                if ch.is_ascii_alphabetic() {
                                        suffix.push(ch);
                                        self.advance();
                                    } else {
                                        break;
                                    }
                            }

                            match suffix.as_str() {
                                "kb" => {
                                    is_size = true;
                                    multiplier = 1024;
                                }

                                "mb" => {
                                    is_size = true;
                                    multiplier = 1024 * 1024;
                                }

                                "gb" => {
                                    is_size = true;
                                    multiplier = 1024 * 1024 * 1024;
                                }

                                _ => {
                                    let extra = vec!["kb", "gb", "mb"];
                                    let mut append = String::new();
                                   
                                    if extra.contains(&suffix.to_lowercase().as_str()) {
                                        append = format!(", did you mean {}", suffix.to_lowercase());
                                    }
                                    
                                    return Err(LexError { message:
                                        format!("Unknown size unit {suffix}{}", append),
                                        line: line, col: col });
                                }
                            }
                        }
                        else {
                            break;
                        }
                    }

                    if is_decimal {
                        let decimal = digits.parse::<f64>().map_err(|_| LexError{
                           message: String::from("Number is too large"),
                           line : line,
                           col : col
                        })?;

                        tokens.push(Token { 
                            kind: TokenKind::Float(decimal), 
                            line: line,
                            col: col });
                        
                    } else if is_size {
                        let base_value = digits.parse::<u128>().map_err(|_| LexError{
                           message: String::from("Number is too large"),
                           line : line,
                           col : col
                        })? ;

                        let digit = base_value.checked_mul(multiplier).ok_or_else(|| LexError {
                                    message: String::from("Number is too large"),
                                    line: line,
                                    col: col,
                                })?;
                        
                        tokens.push(Token { 
                            kind: TokenKind::Size(digit), 
                            line: line,
                            col: col });
                        
                    } else {
                        let digit = digits.parse::<u128>().map_err(|_| LexError{
                           message: String::from("Number is too large"),
                           line : line,
                           col : col
                        })?;
                        
                        tokens.push(Token { 
                            kind: TokenKind::Int(digit), 
                            line: line,
                            col: col });
                    }
                }

                '.' => {
                    let line = self.line;
                    let col = self.col;
                    self.advance();

                    tokens.push(Token { 
                        kind: TokenKind::Dot,
                        line: line,
                        col: col });
                }

                '"' => {
                    let line = self.line;
                    let col = self.col;
                    self.advance(); // consume opening quote
                
                    let mut text = String::new();
                
                    loop {
                        let current = self.peek();
                
                        if current.is_none() || current == Some('\n') {
                            return Err(LexError {
                                message: String::from("Unclosed String, did you forget to add '\"' at the end?"),
                                line,
                                col,
                            });
                        }
                
                        let t = current.unwrap();
                
                        if t == '"' {
                            self.advance(); // consume closing quote
                            break;
                        } else if t == '\\' {
                            self.advance(); // consume backslash
                            let escaped = match self.peek() {
                                Some(ch) => ch,
                                None => return Err(LexError {
                                    message: String::from("Unexpected Character '\\', did you mean '\\n'?"),
                                    line: self.line,
                                    col: self.col,
                                }),
                            };
                
                            match escaped {
                                'n' => text.push('\n'),
                                't' => text.push('\t'),
                                '"' => text.push('"'),
                                '\\' => text.push('\\'),
                                _ => return Err(LexError {
                                    message: format!("Unexpected Character '{escaped}' after \\"),
                                    line: self.line,
                                    col: self.col,
                                }),
                            }
                            self.advance(); // consume the escaped character
                        } else {
                            text.push(t);
                            self.advance(); // consume normal character
                        }
                    }
                
                    tokens.push(Token {
                        kind: TokenKind::Text(text),
                        line,
                        col,
                    });
                }
                
                _ => {
                   return Err(LexError{
                        message: format!("Unexpected Character: {:?}",c),
                        line: self.line,
                        col: self.col
                    });
                }
            }

            //While End
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            col: self.col,
        });

        Ok(tokens)
    }

    //Impl End
}
