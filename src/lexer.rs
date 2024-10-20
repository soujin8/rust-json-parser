#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(f64),
    Null,
    WhiteSpace,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

pub struct LexerError {
    pub msg: String
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string()
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable()
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                Token::WhiteSpace => {}
                _ => {
                    tokens.push(token)
                }
            }
        }

        Ok(tokens)
    }

    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// 数字として使用可能な文字まで読み込む。読み込んだ文字列が数字(`f64`)としてParseに成功した場合Tokenを返す。
    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// 終端文字'\"'まで文字列を読み込む。UTF-16(\u0000~\uFFFF)や特殊なエスケープ文字(e.g. '\t','\n')も考慮する
    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// utf16のバッファが存在するならば連結しておく
    fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
        unimplemented!()
    }
}
