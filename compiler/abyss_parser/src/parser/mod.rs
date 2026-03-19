pub mod handlers;
pub mod precedence;
pub mod rules;
use abyss_diagnostics::DiagnosticEngine;
use abyss_utils::idgen::IdGenerator;

use crate::ast::{Expr, ExprKind, Program};

use crate::parser::engine::PrattEngine;
pub mod engine;

pub struct Parser<'a, 'e, 'i> {
    engine: PrattEngine<'a, 'e, 'i>,
}

impl<'a, 'e, 'i> Parser<'a, 'e, 'i> {
    pub fn new(
        source: &'a str,
        err_handle: &'e mut DiagnosticEngine,
        idgen: &'i mut IdGenerator,
        file_id: u16,
    ) -> Self {
        Self {
            engine: PrattEngine::new(source, err_handle, idgen, file_id),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();
        let mut root_start = 0;
        let mut root_end = 0;

        while !self.engine.is_eof() {
            match self.engine.parse_expression() {
                Ok(expr) => {
                    if program.is_empty() {
                        root_start = expr.span.start;
                    }
                    root_end = expr.span.end;
                    program.push(expr);
                }
                Err(_) => {
                    self.engine.synchronize();
                }
            }

            if self.engine.is_eof() {
                break;
            }

            let current = self.engine.current_token();

            if current.kind == abyss_lexer::token::TokenKind::Comma {
                self.engine.advance();
            } else if current.preceded_by_newline {
                continue;
            } else {
                let span = self.engine.current_span();
                self.engine.report_error(
                    span,
                    "Expected `,` or newline to separate statements.".to_string(),
                );
                self.engine.synchronize();
            }
        }

        Program {
            body: Expr {
                kind: ExprKind::Block(program),
                span: abyss_diagnostics::Span {
                    file_id: self.engine.file_id,
                    start: root_start,
                    end: root_end,
                },
                id: self.engine.next_id(),
            },
        }
    }
}
