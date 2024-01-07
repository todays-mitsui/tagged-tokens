#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Backquote,
    Var(String),
}

impl From<&str> for Token {
    fn from(name: &str) -> Self {
        Token::Var(name.to_string())
    }
}

impl From<String> for Token {
    fn from(name: String) -> Self {
        Token::Var(name)
    }
}
