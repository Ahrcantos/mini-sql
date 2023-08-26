#[derive(Debug)]
pub enum Error {
    UnknownKeyword(String),
    NoMatch,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Comma,
    Equals,
    GreaterThan,
    GreaterThanEquals,
    SmallerThan,
    SmallerThanEquals,
    Whitespace,
    SemiColon,
    Identifier(String),
    String(String),
    Integer(u32),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Select,
    From,
    Insert,
    Into,
    Values,
    Where,
}

pub struct Tokenizer {}

impl Tokenizer {
    pub fn parse(input: &str) -> Vec<Token> {
        let mut output = Vec::new();
        let mut current_position = input;
        let mut offset = 0;

        while !current_position.is_empty() {
            if let Ok((rest, token)) = Self::whitespace((offset, current_position)) {
                current_position = rest;
                offset = token.span.end;
                output.push(token);
                continue;
            }

            if let Ok((rest, token)) = Self::special((offset, current_position)) {
                current_position = rest;
                offset = token.span.end;
                output.push(token);
                continue;
            }

            if let Ok((rest, token)) = Self::identifier((offset, current_position)) {
                current_position = rest;
                offset = token.span.end;
                output.push(token);
                continue;
            }

            if let Ok((rest, token)) = Self::string((offset, current_position)) {
                current_position = rest;
                offset = token.span.end;
                output.push(token);
                continue;
            }

            if let Ok((rest, token)) = Self::number((offset, current_position)) {
                current_position = rest;
                offset = token.span.end;
                output.push(token);
                continue;
            }

            match Self::keyword((offset, current_position)) {
                Ok((rest, token)) => {
                    current_position = rest;
                    offset = token.span.end;
                    output.push(token);
                    continue;
                }
                Err(err) => {
                    dbg!(err);
                    break;
                }
            }
        }

        output
    }

    fn keyword((offset, input): (usize, &str)) -> Result<(&str, Token)> {
        if input.to_lowercase().starts_with("select") {
            return Ok((
                &input[6..],
                Token {
                    span: Span {
                        start: offset,
                        end: offset + 6,
                    },
                    kind: TokenKind::Keyword(Keyword::Select),
                },
            ));
        }

        if input.to_lowercase().starts_with("insert") {
            return Ok((
                &input[6..],
                Token {
                    span: Span {
                        start: offset,
                        end: offset + 6,
                    },
                    kind: TokenKind::Keyword(Keyword::Insert),
                },
            ));
        }

        if input.to_lowercase().starts_with("from") {
            return Ok((
                &input[4..],
                Token {
                    span: Span {
                        start: offset,
                        end: offset + 4,
                    },
                    kind: TokenKind::Keyword(Keyword::From),
                },
            ));
        }

        if input.to_lowercase().starts_with("where") {
            return Ok((
                &input[5..],
                Token {
                    span: Span {
                        start: offset,
                        end: offset + 5,
                    },
                    kind: TokenKind::Keyword(Keyword::Where),
                },
            ));
        }

        if input.to_lowercase().starts_with("values") {
            return Ok((
                &input[6..],
                Token {
                    span: Span {
                        start: offset,
                        end: offset + 6,
                    },
                    kind: TokenKind::Keyword(Keyword::Values),
                },
            ));
        }

        if input.to_lowercase().starts_with("into") {
            return Ok((
                &input[4..],
                Token {
                    span: Span {
                        start: offset,
                        end: offset + 4,
                    },
                    kind: TokenKind::Keyword(Keyword::Into),
                },
            ));
        }

        Err(Error::UnknownKeyword(String::from("idk")))
    }

    fn whitespace((position, input): (usize, &str)) -> Result<(&str, Token)> {
        let start = position;
        let mut end = position;
        let mut current = input;

        for (i, char) in input.chars().enumerate() {
            if i == 0 && !char.is_whitespace() {
                return Err(Error::NoMatch);
            }

            if char.is_whitespace() {
                current = &current[1..];
                end += 1;
            } else {
                return Ok((
                    current,
                    Token {
                        span: Span { start, end },
                        kind: TokenKind::Whitespace,
                    },
                ));
            }
        }

        Ok((
            current,
            Token {
                span: Span { start, end },
                kind: TokenKind::Whitespace,
            },
        ))
    }

    fn special((position, input): (usize, &str)) -> Result<(&str, Token)> {
        if input.get(0..1) == Some(",") {
            return Ok((
                &input[1..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                    kind: TokenKind::Comma,
                },
            ));
        }

        if input.get(0..1) == Some("=") {
            return Ok((
                &input[1..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                    kind: TokenKind::Equals,
                },
            ));
        }

        if input.get(0..1) == Some(";") {
            return Ok((
                &input[1..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                    kind: TokenKind::SemiColon,
                },
            ));
        }

        if input.get(0..2) == Some(">=") {
            return Ok((
                &input[2..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 2,
                    },
                    kind: TokenKind::GreaterThanEquals,
                },
            ));
        }

        if input.get(0..1) == Some(">") {
            return Ok((
                &input[1..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                    kind: TokenKind::GreaterThan,
                },
            ));
        }

        if input.get(0..2) == Some("<=") {
            return Ok((
                &input[2..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 2,
                    },
                    kind: TokenKind::SmallerThanEquals,
                },
            ));
        }

        if input.get(0..1) == Some("<") {
            return Ok((
                &input[1..],
                Token {
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                    kind: TokenKind::SmallerThan,
                },
            ));
        }

        Err(Error::NoMatch)
    }

    fn identifier((position, input): (usize, &str)) -> Result<(&str, Token)> {
        let mut iden = String::new();
        let mut current_position = input;
        let start = position;
        let mut end = position;

        for (i, char) in input.chars().enumerate() {
            if i == 0 && char != '\'' {
                return Err(Error::NoMatch);
            }

            end += 1;
            current_position = &current_position[1..];

            if i != 0 && char == '\'' {
                return Ok((
                    current_position,
                    Token {
                        span: Span { start, end },
                        kind: TokenKind::Identifier(iden),
                    },
                ));
            }

            if char.is_alphabetic() || char == '.' {
                iden.push(char)
            }
        }

        Err(Error::NoMatch)
    }

    fn string((position, input): (usize, &str)) -> Result<(&str, Token)> {
        let mut value = String::new();
        let mut current_position = input;
        let start = position;
        let mut end = position;

        for (i, char) in input.chars().enumerate() {
            if i == 0 && char != '\"' {
                return Err(Error::NoMatch);
            }

            end += 1;
            current_position = &current_position[1..];

            if i != 0 && char == '\"' {
                return Ok((
                    current_position,
                    Token {
                        span: Span { start, end },
                        kind: TokenKind::String(value),
                    },
                ));
            }

            if char != '\"' {
                value.push(char)
            }
        }

        Err(Error::NoMatch)
    }

    fn number((position, input): (usize, &str)) -> Result<(&str, Token)> {
        let mut number = String::new();
        let start = position;
        let mut end = position;
        let mut current = input;

        for (i, char) in input.chars().enumerate() {
            if i == 0 && !char.is_numeric() {
                return Err(Error::NoMatch);
            }

            if !char.is_numeric() {
                break;
            }

            end += 1;
            current = &current[1..];

            number.push(char);
        }

        let n: u32 = number
            .parse()
            .expect("Failed to parse number. Probably too big");

        Ok((
            current,
            Token {
                span: Span { start, end },
                kind: TokenKind::Integer(n),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keyword() {
        let input = "SELECT INSERT";

        let tokens = Tokenizer::parse(input);

        assert_eq!(
            tokens,
            vec![
                Token {
                    span: Span { start: 0, end: 6 },
                    kind: TokenKind::Keyword(Keyword::Select)
                },
                Token {
                    span: Span { start: 6, end: 7 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 7, end: 13 },
                    kind: TokenKind::Keyword(Keyword::Insert),
                }
            ]
        )
    }

    #[test]
    fn test_tokenize_comma() {
        let input = "Select,Insert";
        let tokens = Tokenizer::parse(input);

        assert_eq!(
            tokens,
            vec![
                Token {
                    span: Span { start: 0, end: 6 },
                    kind: TokenKind::Keyword(Keyword::Select)
                },
                Token {
                    span: Span { start: 6, end: 7 },
                    kind: TokenKind::Comma,
                },
                Token {
                    span: Span { start: 7, end: 13 },
                    kind: TokenKind::Keyword(Keyword::Insert)
                },
            ]
        )
    }

    #[test]
    fn test_tokenize_iden() {
        let input = "Select   'id'";

        let tokens = Tokenizer::parse(input);

        assert_eq!(
            tokens,
            vec![
                Token {
                    span: Span { start: 0, end: 6 },
                    kind: TokenKind::Keyword(Keyword::Select)
                },
                Token {
                    span: Span { start: 6, end: 9 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 9, end: 13 },
                    kind: TokenKind::Identifier(String::from("id")),
                }
            ]
        );
    }

    #[test]
    fn test_tokenize_select_with_where() {
        let input = "SELECT 'id' FROM 'users' WHERE 'id' = 2;";
        let tokens = Tokenizer::parse(input);

        assert_eq!(
            tokens,
            vec![
                Token {
                    span: Span { start: 0, end: 6 },
                    kind: TokenKind::Keyword(Keyword::Select)
                },
                Token {
                    span: Span { start: 6, end: 7 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 7, end: 11 },
                    kind: TokenKind::Identifier(String::from("id")),
                },
                Token {
                    span: Span { start: 11, end: 12 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 12, end: 16 },
                    kind: TokenKind::Keyword(Keyword::From),
                },
                Token {
                    span: Span { start: 16, end: 17 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 17, end: 24 },
                    kind: TokenKind::Identifier(String::from("users")),
                },
                Token {
                    span: Span { start: 24, end: 25 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 25, end: 30 },
                    kind: TokenKind::Keyword(Keyword::Where),
                },
                Token {
                    span: Span { start: 30, end: 31 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 31, end: 35 },
                    kind: TokenKind::Identifier(String::from("id")),
                },
                Token {
                    span: Span { start: 35, end: 36 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 36, end: 37 },
                    kind: TokenKind::Equals,
                },
                Token {
                    span: Span { start: 37, end: 38 },
                    kind: TokenKind::Whitespace,
                },
                Token {
                    span: Span { start: 38, end: 39 },
                    kind: TokenKind::Integer(2),
                },
                Token {
                    span: Span { start: 39, end: 40 },
                    kind: TokenKind::SemiColon,
                },
            ]
        )
    }
}
