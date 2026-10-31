mod lexer;
mod tokens;
mod pre_processor;

use std::{fs, io::Write};
use tokens::Token;

fn compile(contents: &str)-> Result<(), Box<dyn std::error::Error>> {
    let stripped_code = pre_processor::remove_comments(contents);
    let mut out_file = fs::File::create(r#"F:\git_repos\c-compiler\main.o"#)?;
    let tokens = Token::from_string(stripped_code);
    let mut out = String::new();
    for token in pre_processor::expand_macros(tokens) {
        out += &token.to_string();
    }

    out_file.write_all(&out.into_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents: &str = &*fs::read_to_string(r#"F:\git_repos\c-compiler\main.c"#).unwrap();
    compile(contents)
}
