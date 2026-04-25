use abyss_diagnostics::Span;
use std::fmt;
use std::{collections::HashMap, rc::Rc};

use crate::core::lexer::{Scanner, Token};

pub type PrefixFn<'a, T> =
    Rc<dyn Fn(&mut DynamicPrattParser<'a, T>, Token<'a>) -> Result<T, String>>;
pub type InfixFn<'a, T> =
    Rc<dyn Fn(&mut DynamicPrattParser<'a, T>, T, Token<'a>) -> Result<T, String>>;

#[derive(Clone)]
pub struct DynamicParseRule<'a, T> {
    pub precedence: u8,
    pub prefix: Option<PrefixFn<'a, T>>,
    pub infix: Option<InfixFn<'a, T>>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Option<Span>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(
                f,
                "Parse error at {}:{}: {}",
                span.file_id, span.start, self.message
            )
        } else {
            write!(f, "Parse error: {}", self.message)
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone)]
pub struct ParserState<'a> {
    pub scanner_off: usize,
    pub current_token: Option<Token<'a>>,
}

pub struct DynamicPrattParser<'a, T> {
    pub scanner: Scanner<'a>,
    pub current_token: Option<Token<'a>>,
    pub rules: HashMap<u32, DynamicParseRule<'a, T>>,
    ignore_tokens: Vec<u32>,
    pub file_id: u16,
}

impl<'a, T: Clone> DynamicPrattParser<'a, T> {
    pub fn new(scanner: Scanner<'a>, file_id: u16) -> Self {
        Self {
            scanner,
            current_token: None,
            rules: HashMap::new(),
            ignore_tokens: Vec::new(),
            file_id,
        }
    }

    pub fn save_state(&self) -> ParserState<'a> {
        ParserState {
            scanner_off: self.scanner.get_offset(),
            current_token: self.current_token.clone(),
        }
    }

    pub fn restore_state(&mut self, state: ParserState<'a>) {
        self.scanner.set_offset(state.scanner_off);
        self.current_token = state.current_token;
    }

    pub fn ignore_token(&mut self, kind_name: &str) {
        let id = self.scanner.rules.id(kind_name);
        self.ignore_tokens.push(id);
    }

    pub fn add_rule<F>(&mut self, kind_name: &str, precedence: u8, callback: F)
    where
        F: Fn(&mut Self, Token<'a>) -> Result<T, String> + 'static,
    {
        let prefix_fn = Rc::new(callback);
        self.register_rule(kind_name, precedence, Some(prefix_fn), None);
    }

    pub fn register_rule(
        &mut self,
        kind_name: &str,
        precedence: u8,
        prefix: Option<PrefixFn<'a, T>>,
        infix: Option<InfixFn<'a, T>>,
    ) {
        let id = self.scanner.rules.id(kind_name);
        let rule = self.rules.entry(id).or_insert(DynamicParseRule {
            precedence: 0,
            prefix: None,
            infix: None,
        });

        if precedence > rule.precedence {
            rule.precedence = precedence;
        }
        if prefix.is_some() {
            rule.prefix = prefix;
        }
        if infix.is_some() {
            rule.infix = infix;
        }
    }

    pub fn advance(&mut self) {
        loop {
            self.current_token = self.scanner.next_token();
            if let Some(ref tk) = self.current_token {
                if !self.ignore_tokens.contains(&tk.kind) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn get_and_bump(&mut self) -> Result<Token<'a>, String> {
        let token = self.current_token.clone().ok_or("Unexpected EOF")?;
        self.advance();
        Ok(token)
    }

    fn error_at_current(&self, message: String) -> String {
        if let Some(ref tk) = self.current_token {
            format!("{} at position {} ('{}')", message, tk.start, tk.text)
        } else {
            format!("{} at end of file", message)
        }
    }

    pub fn parse_expression(&mut self, min_bp: u8) -> Result<T, String> {
        let token = self.get_and_bump()?;

        let rule = self.rules.get(&token.kind).cloned().ok_or_else(|| {
            self.error_at_current(format!("No parse rule for token '{}'", token.text))
        })?;

        let prefix_fn = rule.prefix.ok_or_else(|| {
            self.error_at_current(format!("'{}' cannot start an expression", token.text))
        })?;

        let mut left = prefix_fn(self, token)?;

        loop {
            let next_token = match &self.current_token {
                Some(t) => t.clone(),
                None => break,
            };

            let rule = match self.rules.get(&next_token.kind) {
                Some(r) => r.clone(),
                None => break,
            };

            if rule.precedence <= min_bp {
                break;
            }

            let infix_fn = match rule.infix {
                Some(f) => f,
                None => break,
            };

            let op_token = self.get_and_bump()?;
            left = infix_fn(self, left, op_token)?;
        }

        Ok(left)
    }

    pub fn parse_program(&mut self) -> Result<Vec<T>, String> {
        let mut stmts = Vec::new();
        while self.current_token.is_some() {
            stmts.push(self.parse_expression(0)?);
        }
        Ok(stmts)
    }

    pub fn span_from(&self, start_tk: &Token, end_tk: &Token) -> Span {
        Span {
            file_id: self.file_id,
            start: start_tk.start as u32,
            end: (end_tk.start + end_tk.len) as u32,
        }
    }
}
