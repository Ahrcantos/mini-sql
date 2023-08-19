enum Error {
    UnknownKeyword(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn offset(&mut self, value: usize) {
        self.start += value;
        self.end += value;
    }
}

#[derive(Debug, PartialEq)]
struct Token {
    span: Span,
    kind: TokenKind,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Keyword(Keyword),
    Comma,
    Space,
    Identifier(String),
    String(String),
    Integer(u32),
}

#[derive(Debug, PartialEq)]
enum Keyword {
    Select,
    From,
    Insert,
    Values,
}

struct Tokenizer {}

impl Tokenizer {
    pub fn parse(input: &str) -> Vec<Token> {
        let mut output = Vec::new();
        let mut current_position = input;

        while current_position.len() > 0 {
            if let Ok((rest, mut token)) = Self::parse_keyword(current_position) {
                let offset = input.len() - current_position.len();
                token.span.offset(offset);
                output.push(token);

                current_position = rest;
            } else {
                current_position = &current_position[1..];
            }
        }

        output
    }

    fn parse_keyword(input: &str) -> Result<(&str, Token)> {
        if input.to_lowercase().starts_with("select") {
            return Ok((
                &input[6..],
                Token {
                    span: Span { start: 0, end: 6 },
                    kind: TokenKind::Keyword(Keyword::Select),
                },
            ));
        }

        if input.to_lowercase().starts_with("insert") {
            return Ok((
                &input[6..],
                Token {
                    span: Span { start: 0, end: 6 },
                    kind: TokenKind::Keyword(Keyword::Insert),
                },
            ));
        }

        if input.to_lowercase().starts_with(" ") {
            return Ok((
                &input[1..],
                Token {
                    span: Span { start: 0, end: 1 },
                    kind: TokenKind::Space,
                },
            ));
        }

        return Err(Error::UnknownKeyword(String::from("idk")));
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
                    kind: TokenKind::Space,
                },
                Token {
                    span: Span { start: 7, end: 13 },
                    kind: TokenKind::Keyword(Keyword::Insert),
                }
            ]
        )
    }

    #[test]
    fn test_tokenize_iden() {
        let input = "Select 'id'";

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
                    kind: TokenKind::Space,
                },
                Token {
                    span: Span { start: 7, end: 10 },
                    kind: TokenKind::Identifier(String::from("id")),
                }
            ]
        );
    }
}
