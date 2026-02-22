use abyss_lexer::token::{Token, TokenKind};

use crate::{
    ast::{Expr, ExprKind, Span},
    error::{ParseError, ParseErrorKind},
    parser::{precedence::Precedence, rules::get_rule},
    source_map::SourceMap,
    stream::TokenStream,
};

pub struct PrattEngine<'a> {
    source: &'a str,
    map: SourceMap,
    stream: TokenStream<'a>,
    _errors: Vec<ParseError>,
    last_id: u32,
}

impl<'a> PrattEngine<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            map: SourceMap::new(source),
            stream: TokenStream::new(source),
            _errors: Vec::new(),
            last_id: 0,
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_expression_bp(Precedence::None)
    }

    pub fn parse_expression_bp(&mut self, min_bp: Precedence) -> Result<Expr, ParseError> {
        let token = self.stream.current();
        let rule = get_rule(token.kind);

        let prefix_fn = match rule.prefix {
            Some(func) => func,
            None => {
                return Err(self.make_error(ParseErrorKind::UnexpectedToken {
                    found: token.kind,
                    expected: abyss_lexer::token::TokenKind::Unknown,
                }));
            }
        };

        let mut left = prefix_fn(self)?;

        loop {
            let next_token = self.stream.current();

            if next_token.kind == TokenKind::Eof {
                break;
            }

            let next_rule = get_rule(next_token.kind);

            if next_token.preceded_by_newline {
                if !next_rule.is_soft {
                    break;
                }
            }

            if next_rule.precedence <= min_bp {
                break;
            }

            let infix_fn = match next_rule.infix {
                Some(func) => func,
                None => break,
            };

            left = infix_fn(self, left)?;
        }

        Ok(left)
    }

    pub fn advance(&mut self) {
        self.stream.advance();
    }
    pub fn current_token(&self) -> Token<'a> {
        self.stream.current()
    }

    pub fn consume(&mut self, kind: TokenKind) -> Result<Token<'a>, ParseError> {
        if self.stream.current().kind == kind {
            let token = self.stream.current().clone();
            self.advance();
            Ok(token)
        } else {
            Err(self.make_error(ParseErrorKind::NotAFunction))
        }
    }

    pub fn expect(&mut self, expected_kind: TokenKind) -> Result<Token<'a>, ParseError> {
        let current = self.stream.current().clone();

        if current.kind == expected_kind {
            self.advance();
            Ok(current)
        } else {
            Err(self.make_error(ParseErrorKind::UnexpectedToken {
                found: current.kind,
                expected: expected_kind,
            }))
        }
    }

    pub fn peek(&self) -> Token<'a> {
        self.stream.peek(1)
    }

    pub fn check(&self, kind: TokenKind) -> bool {
        self.stream.current().kind == kind
    }

    pub fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.stream.current().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError {
            kind,
            message: "Test".to_string(),
        }
    }

    pub fn current_span(&self) -> Span {
        let pos = self
            .map
            .find_position(self.stream.current().start, self.source)
            .unwrap();

        Span {
            col: pos.column as u32,
            line: pos.line as u32,
            ..Default::default()
        }
    }

    pub fn new_expr(&self, kind: ExprKind) -> Expr {
        Expr {
            kind,
            span: self.current_span(),
            id: 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.stream.is_eof()
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.last_id;
        self.last_id += 1;
        id
    }
}
