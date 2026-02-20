use abyss_lexer::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    position: usize,
    eof_token: Token<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::Eof {
                break;
            }

            if !Self::is_skippable(&token) {
                tokens.push(token);
            }
        }

        let eof_token = Token::new(TokenKind::Eof, "", 0, 0, false);

        Self {
            source,
            tokens,
            position: 0,
            eof_token,
        }
    }

    fn is_skippable(token: &Token) -> bool {
        matches!(token.kind, TokenKind::Whitespace | TokenKind::Comment)
    }

    pub fn current(&self) -> Token<'a> {
        self.peek(0).clone()
    }

    /// n=0 -> current
    /// n=1 -> next
    /// n=2 -> ...
    pub fn peek(&self, offset: usize) -> Token<'a> {
        if self.position + offset >= self.tokens.len() {
            return self.eof_token;
        }
        self.tokens[self.position + offset]
    }

    pub fn advance(&mut self) {
        if !self.is_eof() {
            self.position += 1;
        }
    }

    pub fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn is(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    pub fn is_peek(&self, kind: TokenKind) -> bool {
        self.peek(1).kind == kind
    }

    pub fn consume(&mut self, kind: TokenKind) -> bool {
        if self.is(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, String> {
        if self.is(kind) {
            let token = self.tokens[self.position];
            self.advance();
            Ok(token)
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                kind,
                self.current().kind
            ))
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }
}
