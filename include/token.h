#include <string>

enum class TokenType :  {
ILLEGAL = "ILLEGAL";
EOF = "EOF";
// Identifiers + literals
IDENT = "IDENT" // add, foobar, x, y, ...
INT = "INT"
// Operators
ASSIGN   = "="
PLUS     = "+"
// Delimiters
COMMA     = ","
SEMICOLON = ";"
LPAREN = "("
RPAREN = ")"
LBRACE = "{"
RBRACE = "}"
// Keywords
// 1343456
       FUNCTION = "FUNCTION"
LET = "LET"
};

struct Token {
    TokenType type;
    std::string literal;
};