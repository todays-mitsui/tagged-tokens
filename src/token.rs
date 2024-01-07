

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Backquote,
    Identifier(String),
}

impl From<&str> for Token {
    fn from(name: &str) -> Self {
        Token::Identifier(name.to_string())
    }
}

impl From<String> for Token {
    fn from(name: String) -> Self {
        Token::Identifier(name)
    }
}
