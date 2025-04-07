use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Context;
use regex::Regex;
use crate::tokens::TokenType;

pub fn lex(path:&Path) -> Result<Vec<TokenType>, anyhow::Error> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    // let source_file = File::open(path).with_context(|| format!("Could not open file: {}", path.display()))?;
    // let reader = BufReader::new(source_file);
     let mut tokens = Vec::new();
    // let chars: Vec<char> = reader.;


    //
   // for line_result in reader.lines() {
      //  let line = line_result.with_context(|| format!("could not read line `{}`", path.display()))?;
        let chars: Vec<char> = content.chars().collect();
        let mut index = 0;

        while index < chars.len() {
            match chars[index] {
                '{' => {
                    tokens.push( TokenType::LEFT_BRACE);
                    index += 1;
                },
                '}' => {
                    tokens.push(TokenType::RIGHT_BRACE);
                    index += 1;
                },
                '(' => {
                    tokens.push(TokenType::LEFT_PAREN);
                    index += 1;
                },
                ')' => {
                    tokens.push(TokenType::RIGHT_PAREN);
                    index += 1;
                },
                ';' => {
                    tokens.push(TokenType::SEMICOLON);
                    index += 1;
                },
                c if c.is_whitespace() => {
                    index += 1;
                    continue
                },
                c if c.is_ascii_digit() => {
                    tokens.push(TokenType::CONSTANT(chars[index].to_digit(10).unwrap() as i32));
                    index += 1;
                },
                c if c.is_ascii_alphabetic() => {
                    let re = Regex::new(r"[a-zA-Z_]\w*\b")?;
                    let line_str: String = chars[index..].iter().collect();
                    
                    for cap in re.find_iter(line_str.as_str()) {
                        tokens.push(TokenType::IDENTIFIER(cap.as_str().to_string()));
                        index += cap.len();
                        break
                    }
                }
                _   => {
                    index += 1;
                    continue
                },
            };
        }
        
 //   }

    Ok(tokens)
}