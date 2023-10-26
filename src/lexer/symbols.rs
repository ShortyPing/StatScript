#[derive(Debug, PartialEq)]
pub enum SymbolType {
    ExclamationMark,  // !
    AtSign,           // @
    Hashtag,          // #
    Dollar,           // $
    Percent,          // %
    Power,            // ^
    Ampersand,        // &
    Asterisk,         // *
    ParenthesisLeft,  // (
    ParenthesisRight, // )
    BracketLeft,      // [
    BracketRight,     // ]
    BraceLeft,        // {
    BraceRight,       // }
    Minus,            // -
    Plus,             // +
    Slash,            // /
    Backslash,        // \
    Equals,           // =
    Colon,            // :
    Semicolon,        // ;
    Dot,              // .
    Comma,            // ,
    QuestionMark,     // ?
    DoubleQuote,      // "
    SingleQuote,      // '
    Backtick,         // `
    Tilde,            // ~
    Pipe,             // |
    TagLeft,          // <
    TagRight,         // >
}

impl SymbolType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '!' => Some(Self::ExclamationMark),
            '@' => Some(Self::AtSign),
            '#' => Some(Self::Hashtag),
            '$' => Some(Self::Dollar),
            '%' => Some(Self::Percent),
            '^' => Some(Self::Power),
            '&' => Some(Self::Ampersand),
            '*' => Some(Self::Asterisk),
            '(' => Some(Self::ParenthesisLeft),
            ')' => Some(Self::ParenthesisRight),
            '[' => Some(Self::BracketLeft),
            ']' => Some(Self::BracketRight),
            '{' => Some(Self::BraceLeft),
            '}' => Some(Self::BraceRight),
            '-' => Some(Self::Minus),
            '+' => Some(Self::Plus),
            '/' => Some(Self::Slash),
            '\\' => Some(Self::Backslash),
            '=' => Some(Self::Equals),
            ':' => Some(Self::Colon),
            ';' => Some(Self::Semicolon),
            '.' => Some(Self::Comma),
            '?' => Some(Self::QuestionMark),
            '"' => Some(Self::DoubleQuote),
            '\'' => Some(Self::SingleQuote),
            '`' => Some(Self::Backtick),
            '~' => Some(Self::Tilde),
            '|' => Some(Self::Pipe),
            '<' => Some(Self::TagLeft),
            '>' => Some(Self::TagLeft),
            _ => None,
        }
    }
}
