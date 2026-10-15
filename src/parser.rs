use crate::{
    ast::{Expression, Program, Statement},
    lexer::{LexError, Lexer, Token},
};

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    current: Token,
}

// for later
#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    UnexpectedToken(Token),
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token().unwrap_or(Token::Eof);

        Self { lexer, current }
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        let previous = std::mem::replace(&mut self.current, Token::Eof);

        self.current = self.lexer.next_token().map_err(ParseError::Lex)?;

        Ok(previous)
    }

    fn expect(&mut self, expected: Token) -> Result<Token, ParseError> {
        if self.current == expected {
            self.advance()
        } else {
            Err(ParseError::UnexpectedToken(self.current.clone()))
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.parse_program()
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        self.expect(Token::Main)?;
        self.expect(Token::CurlyOpen)?;

        let mut main = Vec::new();
        self.skip_separators()?;

        while self.current != Token::CurlyClose {
            if self.current == Token::Eof {
                return Err(ParseError::UnexpectedToken(Token::Eof));
            }

            main.push(self.parse_statement()?);
            self.skip_separators()?;
        }

        self.expect(Token::CurlyClose)?;

        Ok(Program { main })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current {
            Token::Exit => self.parse_exit(),
            _ => Err(ParseError::UnexpectedToken(self.current.clone())),
        }
    }

    fn skip_separators(&mut self) -> Result<(), ParseError> {
        while self.current == Token::Newline || self.current == Token::Semi {
            self.advance()?;
        }

        Ok(())
    }

    // for later
    #[allow(dead_code)]
    fn parse_expression(&mut self) -> Result<crate::ast::Expression, ParseError> {
        match self.current {
            Token::Integer(value) => {
                self.advance()?;
                Ok(crate::ast::Expression::Integer(value))
            }
            _ => Err(ParseError::UnexpectedToken(self.current.clone())),
        }
    }

    fn parse_exit(&mut self) -> Result<Statement, ParseError> {
        self.advance()?;

        let value = match &self.current {
            Token::Integer(value) => {
                let expression = Expression::Integer(*value);
                self.advance()?;
                Some(expression)
            }
            Token::Newline | Token::Semi | Token::CurlyClose => None,
            _ => return Err(ParseError::UnexpectedToken(self.current.clone())),
        };

        if self.current == Token::Newline || self.current == Token::Semi {
            self.advance()?;
        }

        Ok(Statement::Exit(value))
    }
}
