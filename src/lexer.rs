use std::cmp::min;

pub(crate) fn is_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\x0B' | '\x0C' | '\r')
}

pub(crate) fn match_punctuator(token: &str) -> Option<&str> {
    let punctuators = [
        // 3 chars
        "...", "<<=", ">>=",
        // 2 chars
        "->", "++", "--", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "*=", "/=", "%=", "+=",
        "-=", "&=", "^=", "|=", "##",
        // 1 char
        "[", "]", "(", ")", "{", "}", ".", "&", "*", "+", "-", "~", "!", "/", "%", "<", ">", "^",
        "|", "?", ":", ";", "=", ",", "#",
    ];
    (1..=min(token.len(), 3))
        .rev()
        .filter_map(|len| token.get(..len))
        .find(|candidate| punctuators.contains(candidate))
}

pub(crate) fn is_string_const(token: &str) -> bool {
    if token.chars().count() < 2 {
        return false;
    }
    if token.chars().nth(0).unwrap() == '"' && token.chars().nth(1).unwrap() == '"' {
        true;
    }
    false
}

pub(crate) fn is_valid_identifier(token: &str) -> bool {
    let first_char = token.chars().next().unwrap();
    first_char.is_digit(10) || match_punctuator(&*first_char.to_string()).is_some()
}

pub(crate) fn is_keyword(token: &str) -> bool {
    let keywords = [
        "if", "else", "while", "do", "break", "continue", "int", "double", "float", "return",
        "char", "case", "sizeof", "long", "short", "typedef", "switch", "unsigned", "void",
        "static", "struct", "goto", "define", "include",
    ];
    keywords.contains(&token)
}

pub(crate) fn is_integer(token: &str) -> bool {
    for (i, letter) in token.chars().enumerate() {
        if !letter.is_digit(10) && !(letter == '-' && i == 0) {
            return false;
        }
    }
    true
}

pub(crate) fn is_float(token: &str) -> bool {
    let mut has_decimal = false;
    for (i, letter) in token.chars().enumerate() {
        if !letter.is_digit(10) && letter != '.' && !(letter == '-' && i == 0) {
            return false;
        }
        if letter == '.' {
            has_decimal = true;
        }
    }
    has_decimal
}

pub(crate) fn get_line_number(text: &str, byte_index: usize) -> usize {
    if byte_index > text.len() {
        panic!("Line index {} out of bounds!", byte_index);
    }

    text[..byte_index]
        .bytes()
        .filter(|&b| b == b'\n')
        .count() + 1
}