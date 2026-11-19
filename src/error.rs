pub(crate) use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]

pub(crate) enum ErrorType {
    MultiByteCharLiteral,
}

impl ErrorType {
    fn match_to_string(&self) -> String {
        match self {
            &ErrorType::MultiByteCharLiteral => String::from("Cannot have multiple bytes in char literals"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CompilerError {
    pub(crate) error_type: ErrorType,
    pub(crate) line: usize,
    pub(crate) line_text: String,                 // owned, no lifetime
    pub(crate) underline_section: (usize, usize), // offsets within line_text
}

impl CompilerError {
    fn make_error_text(&self) -> String {
        let (start, end) = self.underline_section;
        let text = &self.line_text;

        format!(
            "\x1b[1;31mFatal compiler error\x1b[0m at {}: {}\x1b[1;31;4m{}\x1b[0m{}, with error: {}",
            self.line,
            &text[..start],
            &text[start..end],
            &text[end..],
            self.error_type.match_to_string()
        )
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.make_error_text())
    }
}

impl Error for CompilerError {}