use crate::tokenizer::{Keyword, Token, TokenKind};

#[derive(Debug)]
pub enum Error {
    NoMatch,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Statement {
    Select(SelectStatement),
}

#[derive(Debug)]
pub struct SelectStatement {
    columns: Vec<String>,
    table: String,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Statement> {
        let tokens = self.tokens.as_slice();

        let (_, statement) = Self::select_statement(tokens)?;

        Ok(Statement::Select(statement))
    }

    fn select_statement(input: &[Token]) -> Result<(&[Token], SelectStatement)> {
        let (rest, _) = Self::select_keyword(input)?;
        let (rest, columns) = Self::columns(rest)?;
        let (rest, _) = Self::from_keyword(rest)?;
        let (rest, table) = Self::table_name(rest)?;
        Ok((
            rest,
            SelectStatement {
                columns,
                table,
            },
        ))
    }

    fn table_name(input: &[Token]) -> Result<(&[Token], String)> {
        let mut position = input;

        if let Some(Token { kind: TokenKind::Whitespace, .. }) = position.get(0) {
            position = &position[1..];
        }

        if let Some(Token { kind: TokenKind::Identifier(table), ..}) = position.get(0) {

            match position.get(1) {
                Some(Token { kind: TokenKind::Whitespace, ..}) => Ok((&position[2..], table.clone())),
                _ => Ok((&position[1..], table.clone()))
            }
            

        } else {
            Err(Error::NoMatch)
        }


    }

    fn from_keyword(input: &[Token]) -> Result<(&[Token], ())> {
        if let Some(Token {
            kind: TokenKind::Keyword(Keyword::From),
            ..
        }) = input.get(0)
        {
            Ok((&input[1..], ()))
        } else {
            Err(Error::NoMatch)
        }

    }

    fn select_keyword(input: &[Token]) -> Result<(&[Token], ())> {
        if let Some(Token {
            kind: TokenKind::Keyword(Keyword::Select),
            ..
        }) = input.get(0)
        {
            Ok((&input[1..], ()))
        } else {
            Err(Error::NoMatch)
        }
    }

    fn columns(input: &[Token]) -> Result<(&[Token], Vec<String>)> {
        let mut position = input;
        let mut columns = Vec::new();
        let mut expect_identifier = true;

        while !position.is_empty() {
            let token = position.get(0);

            if let Some(Token {
                kind: TokenKind::Whitespace,
                ..
            }) = token
            {
                position = &position[1..];
                continue;
            }

            if let (
                Some(Token {
                    kind: TokenKind::Identifier(iden),
                    ..
                }),
                true,
            ) = (token, expect_identifier)
            {
                columns.push(iden.clone());
                position = &position[1..];
                expect_identifier = false;
                continue;
            }

            if let (
                Some(Token {
                    kind: TokenKind::Comma,
                    ..
                }),
                false,
            ) = (token, expect_identifier)
            {
                position = &position[1..];
                expect_identifier = true;
                continue;
            }

            if !expect_identifier {
                break;
            }
        }

        if columns.is_empty() {
            return Err(Error::NoMatch);
        }

        match position.get(0) {
            Some(Token {
                kind: TokenKind::Whitespace,
                ..
            }) => Ok((&position[1..], columns)),
            _ => Ok((position, columns)),
        }
    }
}
