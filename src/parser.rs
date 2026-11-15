use crate::parser::ParseErrorType::*;
use crate::tokens::{Token, TokenType};
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parser(tokens: Vec<Token>) -> Result<Vec<Statement>, Vec<ParseError>> {
    let mut tokens = tokens.into_iter().peekable();
    let mut errors = vec![];
    let mut statements = vec![];
    loop {
        match tokens.next().unwrap_or(Token::EOF).get_token_type() {
            TokenType::Questionmark => {
                parse_function_or_declare(tokens);
            }
            TokenType::Eof => break,
            _ => break,
        }
    }
    Ok(statements)
}

fn parse_function_or_declare(
    mut tokens: Peekable<IntoIter<Token>>,
) -> Result<Statement, ParseError> {
    let token = tokens.next().unwrap_or(Token::EOF);
    let line = token.get_line();
    let identifier = match token.get_token_type_consume() {
        TokenType::Identifier(x) => x,
        _ => return ParseError::from_line(line, ExpectedIdentifier),
    };
    let token = tokens.next().unwrap_or(Token::EOF);
    match token.get_token_type() {
        TokenType::OpenSq => {
            let type_token = tokens
                .next()
                .expect("EOF got through sq bracket token check");
            let type_of = match Type::from_token(&type_token) {
                Some(t) => t,
                None => {
                    return ParseError::from_token(&type_token, InvalidTypeAnnotation);
                }
            };
            if let Some(close_sq_check) = tokens.next() {
                if close_sq_check.get_token_type() != &TokenType::CloseSq {
                    return ParseError::from_token(&close_sq_check, ExpectedCloseSq);
                }
            } else {
                panic!("EOF got through type annotation token check");
            };
            if let Some(equals_check) = tokens.next() {
                if equals_check.get_token_type() != &TokenType::Equals {
                    return ParseError::from_token(&equals_check, ExpectedEqualSign);
                }
            } else {
                panic!("EOF got through equals token check");
            };
            return parse_declare(tokens, type_of, identifier);
        }
        TokenType::Equals => {
            let token = tokens.peek().expect("EOF got through equals token check");
            let type_of = match Type::from_token(&token) {
                Some(t) => t,
                None => {
                    let mut tokens_for_type_check = tokens.clone();
                    loop {
                        let token = tokens_for_type_check.next().expect("EOF got through check");
                        match token.get_token_type() {
                            TokenType::Number(_) => {
                                break Type::Num;
                            }
                            TokenType::Bool(_) => {
                                break Type::Bol;
                            }
                            TokenType::Decimal(_) => {
                                break Type::Dec;
                            }
                            TokenType::Eof | TokenType::Semicolon => {
                                return ParseError::from_token(
                                    &token,
                                    CouldNotInferTypeOfDeclaredVar,
                                );
                            }
                            _ => continue,
                        }
                    }
                }
            };
            return parse_declare(tokens, type_of, identifier);
        }
        TokenType::OpenParen => {
            let mut args_and_types = vec![];
            loop {
                let token = tokens
                    .next()
                    .expect("EOF got through open paren token check");
                if token.get_token_type() == &TokenType::CloseParen {
                    break;
                } else if !matches!(token.get_token_type(), TokenType::Identifier(_)) {
                    return ParseError::from_token(&token, ExpectedIdentifier);
                }
                let identifier = match token.get_token_type_consume() {
                    TokenType::Identifier(s) => s,
                    _ => unreachable!(),
                };
                let type_of = tokens
                    .next()
                    .expect("EOF got through identifier token check");
                let Some(type_of) = Type::from_token(&type_of) else {
                    return ParseError::from_token(&type_of, ExpectedTypeAnnotation);
                };
                args_and_types.push((identifier, type_of));
            }
            let token = tokens.next().expect("EOF got through type token check");
            let return_type = match token.get_token_type() {
                TokenType::OpenSq => {
                    let type_token = tokens.next().expect("EOF got through open sq token check");
                    let Some(type_of) = Type::from_token(&type_token) else {
                        return ParseError::from_token(&type_token, ExpectedTypeAnnotation);
                    };
                    let close_sq_check = tokens.next().expect("EOF got through type token check");
                    if close_sq_check.get_token_type() != &TokenType::CloseSq {
                        return ParseError::from_token(&close_sq_check, ExpectedCloseSq);
                    }
                    let open_brace_check =
                        tokens.next().expect("EOF got through close sq token check");
                    if open_brace_check.get_token_type() != &TokenType::OpenBrace {
                        return ParseError::from_token(&open_brace_check, ExpectedOpenBrace);
                    }
                    Some(type_of)
                }
                TokenType::OpenBrace => None,
                _ => return ParseError::from_token(&token, ExpectedOpenSqOrOpenBrace),
            };
            let flag = args_and_types.is_empty();
            let mut args = Some(args_and_types);
            if flag {
                args = None;
            }
            return parse_function(tokens, return_type, identifier, args);
        }
        _ => {
            return ParseError::from_token(&token, ExpectedEqualSignOrOpenParenOrOpenBrace);
        }
    }
}

fn parse_expr(
    expr_tokens: Vec<Token>,
    expr_l: Option<Expression>
) -> Result<Expression, ParseError> {
    let expr_tokens = expr_tokens.into_iter().peekable();
    let tokenl: (Token, u8) = match expr_tokens.next() {
        Some(t) => {
            let s = match t.get_token_type() {
                TokenType::Minus | TokenType::OpenParen | TokenType::Exclamationmark => 3u8,
                TokenType::Multiply | TokenType::Divide | TokenType::Modulo => 2u8,
                TokenType::Bool(_)
                | TokenType::Number(_)
                | TokenType::Decimal(_)
                | TokenType::Identifier(_) => 0u8,
                _ => return ParseError::from_token(&t, InvalidExpression),
            };
            (t, s)
        }
        None => panic!("EOF got through token check"),
    };
    let token1: (Token, u8) = match expr_tokens.next() {
        Some()
    }
    Ok(())
}

fn parse_declare(
    mut tokens: Peekable<IntoIter<Token>>,
    type_of: Type,
    identifier: String,
) -> Result<Statement, ParseError> {
    let mut expr = vec![];
    loop {
        let token = tokens
            .next()
            .expect("EOF got through semicolon token check");
        if token.get_token_type() == &TokenType::Semicolon
            || token.get_token_type() == &TokenType::Eof
        {
            break;
        }
        expr.push(token);
    }
    let value = parse_expr(tokens, expr)?;
    Ok(Statement::Declare {
        identifier,
        value,
        type_of,
    })
}

fn parse_function(
    mut tokens: Peekable<IntoIter<Token>>,
    return_type: Option<Type>,
    identifier: String,
    arguments: Option<Vec<(String, Type)>>,
) -> Result<Statement, ParseError> {
    ()
}

pub struct ParseError {
    type_of: ParseErrorType,
    line_number: u64,
}
impl ParseError {
    fn from_token<T>(token: &Token, error: ParseErrorType) -> Result<T, Self> {
        Err(ParseError {
            type_of: error,
            line_number: token.get_line(),
        })
    }
    fn from_line<T>(line: u64, error: ParseErrorType) -> Result<T, Self> {
        Err(ParseError {
            type_of: error,
            line_number: line,
        })
    }
}

pub enum ParseErrorType {
    ExpectedIdentifier,
    ExpectedTypeAnnotation,
    ExpectedEqualSignOrOpenParenOrOpenBrace,
    ExpectedOpenSqOrOpenBrace,
    ExpectedEqualSign,
    ExpectedCloseSq,
    ExpectedOpenBrace,
    ExpectedValue,
    ExpectedExpression,
    InvalidExpression,
    InvalidTypeAnnotation,
    CouldNotInferTypeOfDeclaredVar,
}

pub enum Statement {
    Declare {
        identifier: String,
        value: Option<Expression>,
        type_of: Type,
    },
    Reassign {
        identifier: String,
        value: Expression,
    },
    Expression(Expression),
    Block(Vec<Statement>),
    If {
        condition: Expression,
        then_do: Vec<Statement>,
        else_do: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        then_do: Vec<Statement>,
    },
    Fn {
        identifier: String,
        arguments: Vec<(String, Type)>,
        then_do: Vec<Statement>,
    },
    Return(Option<Expression>),
}

pub enum Type {
    Num,
    Dec,
    Bol,
}
impl Type {
    fn from_token(token: &Token) -> Option<Type> {
        match token.get_token_type() {
            TokenType::NumType => Some(Type::Num),
            TokenType::DecType => Some(Type::Dec),
            TokenType::BolType => Some(Type::Bol),
            _ => None,
        }
    }
}

pub enum Expression {
    Number(i64),
    Float(f64),
    Bool(bool),
    Identifier(String),
    Unary {
        op: UnaryOp,
        expression: Box<Expression>,
    }, // Unary meaning one expression being operated on
    Binary {
        op: BinaryOp,
        expression0: Box<Expression>,
        expression1: Box<Expression>,
    }, // Binary meaning two expressions
}

pub enum UnaryOp {
    Neg,
    Not,
}

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    And,
    Or,
}
