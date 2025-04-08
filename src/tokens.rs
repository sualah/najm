
#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Identifier(String),
    Constant(i32),
   // RETURN,
}