use crate::{lexer::Token, Value};

#[derive(Debug, Clone)]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> ParserError {
        ParserError {
            msg: msg.to_string(),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    fn parse(&mut self) -> Result<Value, ParserError> {
        let token = self.peek_expect()?.clone();
        let value =
            match token {
                Token::LeftBrace => self.parse_object(),
                Token::LeftBracket => self.parse_array(),
                Token::String(s) => {
                    self.next_expect()?;
                    Ok(Value::String(s))
                }
                Token::Number(s) => {
                    self.next_expect()?;
                    Ok(Value::Number(s))
                }
                Token::Bool(b) => {
                    self.next_expect()?;
                    Ok(Value::Bool(b))
                }
                Token::Null => {
                    self.next_expect()?;
                    Ok(Value::Null)
                }
                _ => return Err(ParserError::new(&format!(
                    "error: a token must start {{ or [ or string or number or bool or null {:?}",
                    token
                ))),
            };
        value
    }

    fn parse_array(&mut self) -> Result<Value, ParserError> {
        todo!()
    }

    fn parse_object(&mut self) -> Result<Value, ParserError> {
        todo!()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn peek_expect(&self) -> Result<&Token, ParserError> {
        self.peek()
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }

    fn next(&mut self) -> Option<&Token> {
        self.tokens.get(self.index);
        self.index += 1;
    }

    fn next_expect(&mut self) -> Result<&Token, ParserError> {
        self.next()
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_object() {}
    #[test]
    fn test_parse_array() {}
    #[test]
    fn test_parse() {}
}
