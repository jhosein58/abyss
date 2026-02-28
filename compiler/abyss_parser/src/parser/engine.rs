use abyss_diagnostics::{DiagnosticEngine, Span};
use abyss_lexer::token::{Token, TokenKind as Tk};

use crate::{
    ast::Expr,
    parser::{precedence::Precedence, rules::get_rule},
    stream::TokenStream,
};

pub struct PrattEngine<'a, 'e> {
    stream: TokenStream<'a>,
    diagnostics: &'e mut DiagnosticEngine,
    pub file_id: u16,
    last_id: u32,
}

impl<'a, 'e> PrattEngine<'a, 'e> {
    pub fn new(source: &'a str, diagnostics: &'e mut DiagnosticEngine, file_id: u16) -> Self {
        Self {
            stream: TokenStream::new(source),
            diagnostics,
            file_id,
            last_id: 0,
        }
    }

    pub fn report_error(&mut self, span: Span, message: String) {
        self.diagnostics.report_error(span, message);
    }

    pub fn report_error_with_hint(&mut self, span: Span, message: String, hint: String) {
        self.diagnostics.report_error_with_hint(span, message, hint);
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ()> {
        self.parse_expression_bp(Precedence::None)
    }

    pub fn parse_expression_bp(&mut self, min_bp: Precedence) -> Result<Expr, ()> {
        let token = self.stream.current();
        let rule = get_rule(token.kind);

        let prefix_fn = match rule.prefix {
            Some(func) => func,
            None => {
                let span = self.current_span();
                self.report_error(
                    span,
                    format!(
                        "Unexpected token `{:?}`. Expected an expression.",
                        token.kind
                    ),
                );
                return Err(());
            }
        };

        let mut left = prefix_fn(self)?;

        loop {
            let next_token = self.stream.current();

            if next_token.kind == Tk::Eof {
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

    pub fn get_and_bump(&mut self) -> Token<'a> {
        let tk = self.current_token();
        self.advance();
        tk
    }

    pub fn consume(&mut self, kind: Tk) -> Result<Token<'a>, ()> {
        if self.stream.current().kind == kind {
            let token = self.stream.current().clone();
            self.advance();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_eof() {
            if self.stream.current().preceded_by_newline {
                return;
            }
            // until we find a statement boundary
            match self.stream.current().kind {
                Tk::While | Tk::Ret | Tk::If | Tk::For | Tk::CBrace => return,
                _ => {}
            }

            self.advance();
        }
    }

    pub fn expect(&mut self, expected_kind: Tk) -> Result<Token<'a>, ()> {
        let current = self.stream.current().clone();

        if current.kind == expected_kind {
            self.advance();
            Ok(current)
        } else {
            let span = self.current_span();
            self.report_error(
                span,
                format!("Expected `{}`, but found `{}`", expected_kind, current.kind),
            );

            Err(())
        }
    }

    pub fn peek(&self) -> Token<'a> {
        self.stream.peek(1)
    }

    pub fn check(&self, kind: Tk) -> bool {
        self.stream.current().kind == kind
    }

    pub fn match_token(&mut self, kind: Tk) -> bool {
        if self.stream.current().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn current_span(&self) -> Span {
        let tk = self.stream.current();

        Span {
            file_id: self.file_id,
            start: tk.start as u32,
            end: (tk.start + tk.len) as u32,
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
