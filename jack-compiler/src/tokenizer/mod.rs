mod test;
mod token_stream;

use lazy_static::lazy_static;
use regex::Regex;

use super::errors::TokenizerError;
use super::token::{Token, TokenKind};
pub(crate) use token_stream::{TokenResult, TokenStream};

pub struct Source {
    pub content: String,
    pub name: String,
}

pub struct Tokenizer;

macro_rules! tokenize_keyword_arm {
    ($data:ident, $len: expr, $($l:literal),+) => {
        if match $data.chars().skip($len).next() {
            Some(c) => !c.is_ascii_alphabetic(),
            None => true
        } && ($($data.starts_with($l))||+) {
            return Ok(
                (Token {
                    kind: TokenKind::Keyword,
                    value: $data[..$len].to_string()
                }, $len)
            )
        }
    }
}

type TokenizationResult = Result<(Token, usize), TokenizerError>;

impl Tokenizer {
    pub fn stream<'a>(source: &'a Source) -> TokenStream<'a> {
        lazy_static! {
            // BUG: This is just a simple tokenizer. We assume that the comment syntax won't show up in string literals.
            static ref REMOVE_COMMENTS_AND_LONG_SPACES: Regex =
                Regex::new(r#"/\*(.|\n)*?\*/|//.*|\s+"#).unwrap();
        }
        let Source {
            content: source,
            name: _,
        } = source;
        TokenStream::new(REMOVE_COMMENTS_AND_LONG_SPACES.replace_all(&source, " "))
    }

    fn tokenize_keyword(data: &str) -> TokenizationResult {
        // 21 keywords
        tokenize_keyword_arm!(data, 2, "do", "if");
        tokenize_keyword_arm!(data, 3, "var", "int", "let");
        tokenize_keyword_arm!(data, 4, "char", "void", "true", "null", "this", "else");
        tokenize_keyword_arm!(data, 5, "class", "field", "false", "while");
        tokenize_keyword_arm!(data, 6, "method", "static", "return");
        tokenize_keyword_arm!(data, 7, "boolean");
        tokenize_keyword_arm!(data, 8, "function");
        tokenize_keyword_arm!(data, 11, "constructor");
        Err(TokenizerError::InvalidKeyword)
    }

    fn tokenize_integer(data: &str) -> TokenizationResult {
        let num: String = data.chars().take_while(char::is_ascii_digit).collect();
        // Jack integer constant: [0, 32767]
        if num.parse::<i16>().is_ok() {
            let len = num.len();
            Ok((
                Token {
                    kind: TokenKind::IntegerConstant,
                    value: num,
                },
                len,
            ))
        } else {
            Err(TokenizerError::IntegerOutOfRange(num))
        }
    }

    fn tokenize_string(data: &str) -> TokenizationResult {
        // Jack string literal: there is no \n or " inside literal.
        let mut iter = data.chars();
        let begin = iter.next().ok_or(TokenizerError::UnexpectedEOF)?;

        if begin != '"' {
            return Err(TokenizerError::UnexpectedCharacter(begin, "\"".to_string()));
        }
        let literal: String = iter.take_while(|c| *c != '"').collect();
        let len = literal.len() + 2;
        let end = data
            .chars()
            .skip(len - 1)
            .next()
            .ok_or(TokenizerError::UnexpectedEOF)?;
        if end != '"' {
            return Err(TokenizerError::UnexpectedCharacter(end, "\"".to_string()));
        }
        Ok((
            Token {
                kind: TokenKind::StringConstant,
                value: literal,
            },
            len,
        ))
    }

    fn tokenize_identifier(data: &str) -> TokenizationResult {
        // Jack identifier: A seq of letters, digits, and underscore, not starting with a digit

        let begin = data.chars().next().ok_or(TokenizerError::UnexpectedEOF)?;
        if begin.is_ascii_digit() {
            return Err(TokenizerError::IdentifierStartsWithDigit);
        }

        let identifier: String = data
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        let len = identifier.len();
        Ok((
            Token {
                kind: TokenKind::Identifier,
                value: identifier,
            },
            len,
        ))
    }

    fn tokenize_one_token(data: &str) -> TokenizationResult {
        let begin = data.chars().next().ok_or(TokenizerError::UnexpectedEOF)?;
        match begin {
            symbol @ ('{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*'
            | '/' | '&' | '|' | '<' | '>' | '=' | '~') => Ok((
                Token {
                    kind: TokenKind::Symbol,
                    value: symbol.to_string(),
                },
                1,
            )),
            digit if digit.is_ascii_digit() => Self::tokenize_integer(data),
            '"' => Self::tokenize_string(data),
            char if char.is_ascii_alphanumeric() || char == '_' => {
                Self::tokenize_keyword(data).or_else(|_| Self::tokenize_identifier(data))
            }
            c => Err(TokenizerError::UnexpectedCharacter(
                c,
                "a valid character".to_string(),
            )),
        }
    }
}
