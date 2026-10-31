use std::cmp::min;
use crate::lexer;

#[derive(PartialEq, Clone)]
pub(crate) enum TokenType {
    Punctuator,
    IntConstant,
    StringConstant,
    FloatConstant,
    CharConstant,
    Keyword,
    Operator,
    Identifier
}

impl TokenType {
    fn classify(s: &str) -> TokenType {
        if lexer::is_keyword(s)                       { return TokenType::Keyword }
        if lexer::match_punctuator(s).is_some()       { return TokenType::Punctuator }
        if lexer::is_integer(s)                       { return TokenType::IntConstant }
        if lexer::is_float(s)                         { return TokenType::FloatConstant }
        if lexer::is_valid_identifier(s)              { return TokenType::Identifier }
        if lexer::is_string_const(s)                  { return TokenType::StringConstant }
        panic!("Unknown token while parsing: {}", s)
    }

    fn to_string(&self) -> String {
        match self {
            TokenType::Punctuator => "Punctuator".to_string(),
            TokenType::IntConstant => "IntConstant".to_string(),
            TokenType::StringConstant => "StringConstant".to_string(),
            TokenType::FloatConstant => "FloatConstant".to_string(),
            TokenType::CharConstant => "CharConstant".to_string(),
            TokenType::Keyword => "Keyword".to_string(),
            TokenType::Operator => "Operator".to_string(),
            TokenType::Identifier => "Identifier".to_string(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Token {
    pub token: String,
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    fn new(token: String, token_type: TokenType, line: usize) -> Self {
        Self { token, token_type, line }
    }

    pub(crate) fn from_string(code: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut i = 0;
        while i < code.len() {
            let mut character = code.chars().nth(i).unwrap();
            let punctuator = lexer::match_punctuator(&code[i..min(i+3, code.len())]);
            if punctuator.is_some() {
                tokens.push(Token::new(punctuator.unwrap().parse().unwrap(), TokenType::Punctuator, lexer::get_line_number(&code, i)));
                i += punctuator.unwrap().len();
                continue;
            }
            if (character.is_alphanumeric() || character == '_') && !(character == '"') {
                let mut j = i;
                let mut is_int = character.is_ascii_digit();
                let mut isfloat = character.is_ascii_digit();
                while character.is_alphanumeric() || character == '_' || character == '.' {
                    if !character.is_ascii_digit() {
                        is_int = false;
                    }
                    if character == '.' && is_int {
                        isfloat = true;
                        is_int = false;
                    }
                    j += 1;
                    character = code.chars().nth(j).unwrap();
                }
                let token = code[i..j].to_string();
                if lexer::is_keyword(&token) {
                    tokens.push(Token::new(token, TokenType::Keyword, lexer::get_line_number(&code, i)));
                } else if is_int {
                    tokens.push(Token::new(token, TokenType::IntConstant, lexer::get_line_number(&code, i)));
                } else if isfloat {
                    tokens.push(Token::new(token, TokenType::FloatConstant, lexer::get_line_number(&code, i)));
                } else {
                    tokens.push(Token::new(token, TokenType::Identifier, lexer::get_line_number(&code, i)));
                }

                i = j;
                continue;
            }
            if character == '"' {
                let mut j = i + 1;
                character = code.chars().nth(j).unwrap();
                while character != '"' {
                    if j > code.len() {
                        panic!("Unclosed string literal");
                    }
                    j += 1;
                    character = code.chars().nth(j).unwrap();
                }
                let token = code[i..=j].to_string();
                tokens.push(Token::new(token, TokenType::StringConstant, lexer::get_line_number(&code, i)));
                i = j + 1;
                continue;
            }
            if character == "'".chars().next().unwrap() {
                let mut j = i + 1;
                character = code.chars().nth(j).unwrap();
                while character != "'".chars().next().unwrap() {
                    if j > code.len() {
                        panic!("Unclosed char literal");
                    }
                    j += 1;
                    character = code.chars().nth(j).unwrap();
                }
                let token = code[i..=j].to_string();
                tokens.push(Token::new(token, TokenType::CharConstant, lexer::get_line_number(&code, i)));
                i = j + 1;
                continue;
            }

            i += 1;
        }
        tokens
    }

    pub(crate) fn to_string(&self) -> String {
        r#"""#.to_owned() + self.token.as_str() + r#"" is a "# + &*self.token_type.to_string() + " on line " + &*self.line.to_string() + "\n"
    }
}