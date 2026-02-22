pub mod handlers;
pub mod precedence;
pub mod rules;
use crate::ast::{Expr, ExprKind, Program, Span};

use crate::error::ParseError;
use crate::parser::engine::PrattEngine;
pub mod engine;

pub struct Parser<'a> {
    engine: PrattEngine<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            engine: PrattEngine::new(source),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut program = Vec::new();

        while !self.engine.is_eof() {
            let expr = self.engine.parse_expression()?;
            program.push(expr);
        }

        Ok(Program {
            body: Expr {
                kind: ExprKind::Block(program),
                span: Span {
                    ..Default::default()
                },
                id: 0,
            },
        })
    }
}
