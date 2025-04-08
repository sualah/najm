use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, bail, Context};
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
                    tokens.push( TokenType::LeftBrace);
                    index += 1;
                },
                '}' => {
                    tokens.push(TokenType::RightBrace);
                    index += 1;
                },
                '(' => {
                    tokens.push(TokenType::LeftParen);
                    index += 1;
                },
                ')' => {
                    tokens.push(TokenType::RightParen);
                    index += 1;
                },
                ';' => {
                    tokens.push(TokenType::Semicolon);
                    index += 1;
                },
                c if c.is_whitespace() => {
                    index += 1;
                    continue;
                },
                c if c.is_ascii_digit() => {
                    let re = Regex::new(r"[0-9]+\b")?;
                    let line_str: String = chars[index..].iter().collect();
                    let mut matched = false;

                    for cap in re.find_iter(line_str.as_str()) {
                        let int_str = cap.as_str();

                        //  tokens.push(TokenType::Identifier(cap.as_str().to_string()));
                        let value: i32 = int_str.parse() // try parse to i32
                            .map_err(|e| anyhow::anyhow!("Failed to parse constant '{}': {}", int_str, e))?;

                        println!("constant is {}", value);
                        tokens.push(TokenType::Constant(value));
                      //  println!("constant is {}", cap.as_str());
                      //  tokens.push(TokenType::Constant(cap.as_str().to_string().parse()?));
                        index += cap.end();
                        matched = true;
                        break;
                    }
                    
                    if !matched {
                      //  index += 1;
                       // bail!("no match found for character `{}`", chars[index - 1]);
                        std::process::exit(1);
                    }
                    // tokens.push(TokenType::Constant(chars[index].to_digit(10).unwrap() as i32));
                    // index += 1;
                },
                c if c.is_ascii_alphabetic() => {
                    let re = Regex::new(r"[a-zA-Z_]\w*\b")?;
                    let line_str: String = chars[index..].iter().collect();

                    for cap in re.find_iter(line_str.as_str()) {
                        tokens.push(TokenType::Identifier(cap.as_str().to_string()));
                        index += cap.len();
                        break;
                    }
                }
                _   => {
                    println!("no match found for character `{}`", chars[index]);
                   // std::process::exit(-1);
                     index += 1;
                     continue;
                    ;
                  //  bail!("no match found for character `{}`", chars[index]);
                },
            };
        }

 //   }

    Ok(tokens)
}