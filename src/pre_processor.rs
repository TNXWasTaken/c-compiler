use crate::tokens::Token;
use crate::tokens::TokenType;
use std::collections::HashMap;

pub(crate) fn remove_comments(code: &str) -> String {
    code.lines().map(|line|
        line.split("//").next().unwrap().to_owned() + "\n"
    ).collect::<String>().split("/*").map(|comment|
        if comment.contains("*/")
        {""}
        else
        {comment}
    ).collect()
}

pub(crate) fn expand_macros(tokens: Vec<Token>) -> Vec<Token> {
    let mut macros: HashMap<String, Vec<Token>> = HashMap::new();
    let mut output = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        // A directive is a '#' followed by 'define' on the same line
        if token.token == "#"
            && iter.peek().is_some_and(|t| t.token == "define" && t.line == token.line)
        {
            iter.next(); // consume 'define'
            let name = iter.next().expect("#define needs a name");

            // The body is every remaining token on the same line
            let mut body = Vec::new();
            while let Some(t) = iter.next_if(|t| t.line == token.line) {
                body.push(t);
            }
            macros.insert(name.token, body);
            continue; // don't emit the directive itself
        }

        expand(token, &macros, &mut Vec::new(), &mut output);
    }
    output
}

fn expand(
    token: Token,
    macros: &HashMap<String, Vec<Token>>,
    active: &mut Vec<String>, // macros currently being expanded
    out: &mut Vec<Token>,
) {
    if token.token_type == TokenType::Identifier {
        if let Some(body) = macros.get(&token.token) {
            if !active.contains(&token.token) {
                active.push(token.token.clone());
                for t in body {
                    expand(t.clone(), macros, active, out); // bodies can contain macros
                }
                active.pop();
                return;
            }
        }
    }
    out.push(token);
}