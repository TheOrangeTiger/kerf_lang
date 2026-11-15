#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    line_number: u64,
}
impl Token {
    pub fn new(token_type: TokenType, line_number: u64) -> Self {
        Token {
            token_type,
            line_number,
        }
    }
    pub fn get_line(&self) -> u64 {
        self.line_number
    }
    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn get_token_type_consume(self) -> TokenType {
        self.token_type
    }
    pub const EOF: Token = Token {
        token_type: TokenType::Eof,
        line_number: 0,
    };
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidToken(String, u64),
    InvalidNumber(String, u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier(String),
    Number(i64),
    NumType,
    Decimal(f64),
    DecType,
    Bool(bool),
    BolType,
    If,
    Elif,
    Else,
    While,
    Return,
    IsEqualTo,
    IsNotEqualTo,
    IsGreaterThan,
    IsLessThan,
    IsGreaterThanOrEqualTo,
    IsLessThanOrEqualTo,
    And,
    Or,
    Equals,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Multiply,
    MultiplyEqual,
    Divide,
    DivideEqual,
    Modulo,
    ModuloEqual,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenSq,
    CloseSq,
    Questionmark,
    Exclamationmark,
    Semicolon,
    Comma,
    Eof,
}
impl TokenType {
    pub fn from_word(word: String) -> Result<TokenType, String> {
        match word.as_str() {
            "if" => Ok(TokenType::If),
            "elif" => Ok(TokenType::Elif),
            "else" => Ok(TokenType::Else),
            "while" => Ok(TokenType::While),
            "return" => Ok(TokenType::Return),
            "true" => Ok(TokenType::Bool(true)),
            "false" => Ok(TokenType::Bool(false)),
            "num" => Ok(TokenType::NumType),
            "dec" => Ok(TokenType::DecType),
            "bol" => Ok(TokenType::BolType),
            x if x.is_ascii() => Ok(TokenType::Identifier(x.to_string())),
            _ => Err(word),
        }
    }
}
