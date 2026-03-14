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

        while !self.engine.is_eof() {
            match self.engine.parse_expression() {
                Ok(expr) => {
                    program.push(expr);
                }
                Err(_) => {
                    self.engine.synchronize();
                }
            }
        }

        Program {
            body: Expr {
                kind: ExprKind::Block(program),
                span: abyss_diagnostics::Span {
                    file_id: 0,
                    start: 0,
                    end: 0,
                },
                id: 0,
            },
        }
    }
}
