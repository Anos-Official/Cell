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
}

//[[
// Structs
// ]]

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
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

    pub fn panic(&self, c: char) {
        panic!(
            "Unexpected Character Found at line: {},  col: {}, Lexer Position : {}, Character: {}",
            self.line, self.line, self.pos, c
        );
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
    pub fn tokenize(&mut self) -> Vec<Token> {
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
                    if self.peek_at(1) == Some('-') {
                        //Is it a multiline comment?
                        if self.peek_at(2) == Some('[') && self.peek_at(3) == Some('[') {
                            //Consume --[[
                            for _ in 0..4 {
                                self.advance();
                            }

                            //Is this the end of multiline?, if not keep going
                            while self.peek().is_some()
                                && !(self.peek() == Some(']')
                                    && self.peek_at(1) == Some(']')
                                    && self.peek_at(2) == Some('-')
                                    && self.peek_at(3) == Some('-'))
                            {
                                self.advance();
                            }

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
                        let line = self.line;
                        let col = self.col;
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
                        self.panic(c);
                    }
                }

                '0'..='9' => {
                    let line = self.line;
                    let col = self.col;

                    let mut digits = String::new();

                    while let Some(d) = self.peek() {
                        if !d.is_ascii_digit() {
                            break;
                        }

                        digits.push(d);
                        self.advance();
                    }
                    
                    let digit = digits.parse::<u128>().unwrap();

                    tokens.push(Token { 
                        kind: TokenKind::Int(digit as u128), 
                        line: line,
                        col: col });
    
                }
                
                _ => {
                    self.panic(c);
                }
            }

            //While End
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            col: self.col,
        });
        tokens
    }

    //Impl End
}
