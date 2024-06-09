pub use crate::pos::{Position, TokenPosition};

#[derive(Debug, Clone)]
pub struct Token {
    t_type: TokenType,
    value: String,
    pos: TokenPosition
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Eof,
    Whitespace,

    Name,

    Number,

    /// (
    OpenBrace,
    /// )
    CloseBrace,

    /// [
    OpenSquareBrace,
    /// ]
    CloseSquareBrace,

    /// {
    OpenCurlyBrace,
    /// }
    CloseCurlyBrace,

    /// =
    Equals,

    /// |
    Pipe,

    /// >
    Greater,

    /// <
    Less,

    /// !
    Exclamation,

    /// ?
    Question,

    /// @
    At,

    /// $
    Dollar,

    /// &
    And,

    /// %
    Percent,

    /// /
    Slash,

    /// \
    Backslash,

    /// :
    Colon,

    /// ;
    Semicolon,

    /// +
    Plus,

    /// -
    Minus,

    /// *
    Star,

    /// .
    Dot,

    /// ,
    Comma,

    /// #
    Hashtag,

    /// ~
    Tilde,

    /// '
    Apostrophe,

    /// ^
    Caret,

    /// °
    Degree,

    /// "
    QuotationMark,

    /// _
    Underscore,

    /// `
    Backtick,

    /// ´
    Prime
}

impl Token {
    pub const fn new(t_type: TokenType, value: String, pos: TokenPosition) -> Self {
        Self { t_type, value, pos }
    }

    pub const fn eof() -> Self {
        let pos = Position::new(0, 0);
        Self::new(TokenType::Eof, String::new(), TokenPosition::new(pos, pos))
    }

    pub fn t_type(&self) -> TokenType {
        self.t_type
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn pos(&self) -> TokenPosition {
        self.pos
    }
}