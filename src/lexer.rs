#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Main,
    Exit,
    Integer(i32),
    CurlyOpen,
    CurlyClose,
    Semi,
    Newline,
    Eof,
}

#[derive(Debug)]
pub struct LexError {
    // for later
    #[allow(dead_code)]
    pub character: char,
}

pub struct Lexer<'src> {
    chars: std::str::Chars<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn read_word(&mut self, first: char) -> Result<Token, LexError> {
        let mut word = String::from(first);

        while let Some(character) = self.chars.clone().next() {
            if !character.is_ascii_alphanumeric() && character != '_' {
                break;
            }
            word.push(character);
            self.chars.next();
        }

        let token = match word.as_str() {
            "main" => Token::Main,
            "exit" => Token::Exit,
            _ => return Err(LexError { character: first }),
        };

        Ok(token)
    }

    fn read_integer(&mut self, first: char) -> Result<Token, LexError> {
        let mut digits = String::from(first);

        while let Some(character) = self.chars.clone().next() {
            if !character.is_ascii_digit() {
                break;
            }
            digits.push(character);
            self.chars.next();
        }

        match digits.parse::<i32>() {
            Ok(value) => Ok(Token::Integer(value)),
            Err(_) => Err(LexError { character: first }),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        loop {
            let Some(character) = self.next_char() else {
                return Ok(Token::Eof);
            };

            let token = match character {
                ' ' | '\t' | '\r' => continue,
                '\n' => Token::Newline,
                '{' => Token::CurlyOpen,
                '}' => Token::CurlyClose,
                ';' => Token::Semi,
                '0'..='9' => return self.read_integer(character),
                'a'..='z' | 'A'..='Z' | '_' => return self.read_word(character),
                _ => return Err(LexError { character }),
            };

            return Ok(token);
        }
    }
}
