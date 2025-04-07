
#[derive(Debug, PartialEq)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    SEMICOLON,
    IDENTIFIER(String),
    CONSTANT(i32),
    RETURN,
}