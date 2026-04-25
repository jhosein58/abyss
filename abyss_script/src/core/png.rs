use abyss_diagnostics::Span;
use std::{collections::HashMap, rc::Rc};

use crate::core::{
    lexer::{Scanner, Token},
    parser::DynamicPrattParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum HoleKind {
    Expr,
    Stmt,
    Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    ZeroOrMore, // *
    OneOrMore,  // +
    ZeroOrOne,  // ?
}

#[derive(Debug, Clone)]
pub enum PatternNode {
    Literal(String),
    Hole(String, HoleKind),
    Group {
        inner: Vec<PatternNode>,
        separator: Option<String>,
        quantifier: Quantifier,
    },
    Choice(Vec<Vec<PatternNode>>), // $< A | B >
}

#[derive(Debug, Clone)]
pub enum SyntaxVar<T> {
    Expr(T),
    Ident(String),
    List(Vec<SyntaxVar<T>>),
}

pub struct SyntaxCtx<T> {
    pub vars: HashMap<String, SyntaxVar<T>>,
    pub start_span: u32,
    pub end_span: u32,
    pub file_id: u16,
}

impl<T: Clone> SyntaxCtx<T> {
    pub fn try_get_node(&self, name: &str) -> Option<T> {
        match self.vars.get(name) {
            Some(SyntaxVar::Expr(n)) => Some(n.clone()),
            _ => None,
        }
    }

    pub fn get_node(&self, name: &str) -> T {
        self.try_get_node(name)
            .unwrap_or_else(|| panic!("Expected Expr for '{}'", name))
    }

    pub fn try_get_ident(&self, name: &str) -> Option<String> {
        match self.vars.get(name) {
            Some(SyntaxVar::Ident(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_ident(&self, name: &str) -> String {
        self.try_get_ident(name)
            .unwrap_or_else(|| panic!("Expected Ident for '{}'", name))
    }

    pub fn try_get_node_list(&self, name: &str) -> Option<Vec<T>> {
        match self.vars.get(name) {
            Some(SyntaxVar::List(l)) => Some(
                l.iter()
                    .map(|v| match v {
                        SyntaxVar::Expr(e) => e.clone(),
                        _ => panic!(
                            "List '{}' does not contain Exprs (It might be nested unexpectedly)",
                            name
                        ),
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    pub fn get_node_list(&self, name: &str) -> Vec<T> {
        self.try_get_node_list(name).unwrap_or_else(|| Vec::new())
    }

    pub fn span(&self) -> Span {
        Span {
            file_id: self.file_id,
            start: self.start_span,
            end: self.end_span,
        }
    }
}

pub trait DynamicSyntaxMagic<'a, T: Clone + 'static> {
    fn define<F>(&mut self, pattern: &str, precedence: u8, callback: F)
    where
        F: Fn(SyntaxCtx<T>) -> T + 'static;

    fn define_expr<F>(&mut self, pattern: &str, precedence: u8, callback: F)
    where
        F: Fn(SyntaxCtx<T>) -> T + 'static;

    fn define_stmt<F>(&mut self, pattern: &str, callback: F)
    where
        F: Fn(SyntaxCtx<T>) -> T + 'static;
}

impl<'a, T: Clone + 'static> DynamicSyntaxMagic<'a, T> for DynamicPrattParser<'a, T> {
    fn define<F>(&mut self, pattern: &str, precedence: u8, callback: F)
    where
        F: Fn(SyntaxCtx<T>) -> T + 'static,
    {
        let nodes = parse_magic_pattern(pattern);
        if nodes.is_empty() {
            return;
        }

        register_literals(&mut self.scanner, &nodes);
        let callback = Rc::new(callback);
        let nodes_clone = nodes.clone();

        match &nodes[0] {
            PatternNode::Literal(trigger_text) => {
                let trigger_id = format!("Token_Auto_{}", trigger_text);
                let cb = Rc::new(
                    move |parser: &mut DynamicPrattParser<'a, T>, start_token: Token<'a>| {
                        let mut ctx_vars = HashMap::new();
                        let mut end_span = (start_token.start + start_token.len) as u32;
                        eval_pattern_nodes(
                            parser,
                            &nodes_clone[1..],
                            &mut ctx_vars,
                            None,
                            &mut end_span,
                        )?;
                        Ok(callback(SyntaxCtx {
                            vars: ctx_vars,
                            start_span: start_token.start as u32,
                            end_span,
                            file_id: parser.file_id,
                        }))
                    },
                );
                self.register_rule(&trigger_id, precedence, Some(cb), None);
            }
            PatternNode::Hole(left_name, HoleKind::Expr) => {
                let left_name_owned = left_name.clone();
                let trigger_text = match &nodes[1] {
                    PatternNode::Literal(t) => t,
                    _ => panic!(
                        "Infix rules MUST have a literal operator as the second item (e.g. ':left + :right')"
                    ),
                };
                let trigger_id = format!("Token_Auto_{}", trigger_text);

                let cb = Rc::new(
                    move |parser: &mut DynamicPrattParser<'a, T>, left: T, op_token: Token<'a>| {
                        let mut ctx_vars = HashMap::new();
                        ctx_vars.insert(left_name_owned.clone(), SyntaxVar::Expr(left.clone()));
                        let mut end_span = (op_token.start + op_token.len) as u32;
                        eval_pattern_nodes(
                            parser,
                            &nodes_clone[2..],
                            &mut ctx_vars,
                            None,
                            &mut end_span,
                        )?;
                        Ok(callback(SyntaxCtx {
                            vars: ctx_vars,
                            start_span: op_token.start as u32,
                            end_span,
                            file_id: parser.file_id,
                        }))
                    },
                );
                self.register_rule(&trigger_id, precedence, None, Some(cb));
            }
            _ => panic!("Pattern must start with a Literal (prefix) or an :Expr hole (infix)"),
        }
    }

    fn define_expr<F>(&mut self, pattern: &str, precedence: u8, callback: F)
    where
        F: Fn(SyntaxCtx<T>) -> T + 'static,
    {
        self.define(pattern, precedence, callback);
    }

    fn define_stmt<F>(&mut self, pattern: &str, callback: F)
    where
        F: Fn(SyntaxCtx<T>) -> T + 'static,
    {
        self.define(pattern, 0, callback);
    }
}

fn eval_pattern_nodes<'a, T: Clone>(
    parser: &mut DynamicPrattParser<'a, T>,
    nodes: &[PatternNode],
    ctx_vars: &mut HashMap<String, SyntaxVar<T>>,
    stop_token_text: Option<&str>,
    end_span: &mut u32,
) -> Result<(), String> {
    let mut idx = 0;

    while idx < nodes.len() {
        match &nodes[idx] {
            PatternNode::Hole(name, kind) => match kind {
                HoleKind::Expr => {
                    let expr = parser.parse_expression(0)?;
                    ctx_vars.insert(name.clone(), SyntaxVar::Expr(expr));
                }
                HoleKind::Stmt => {
                    let stmt = parser.parse_expression(0)?;
                    ctx_vars.insert(name.clone(), SyntaxVar::Expr(stmt));
                }
                HoleKind::Ident => {
                    let tk = parser.get_and_bump()?;
                    *end_span = (tk.start + tk.len) as u32;
                    ctx_vars.insert(name.clone(), SyntaxVar::Ident(tk.text.to_string()));
                }
            },
            PatternNode::Literal(text) => {
                let next = parser.get_and_bump()?;
                if next.text != text {
                    return Err(format!(
                        "Expected '{}' but found '{}' at position {}",
                        text, next.text, next.start
                    ));
                }
                *end_span = (next.start + next.len) as u32;
            }
            PatternNode::Choice(branches) => {
                let mut matched = false;

                for branch in branches {
                    let state = parser.save_state();
                    let mut local_ctx = HashMap::new();
                    let mut local_end_span = *end_span;

                    if eval_pattern_nodes(
                        parser,
                        branch,
                        &mut local_ctx,
                        stop_token_text,
                        &mut local_end_span,
                    )
                    .is_ok()
                    {
                        *end_span = local_end_span;
                        ctx_vars.extend(local_ctx);
                        matched = true;
                        break;
                    } else {
                        parser.restore_state(state);
                    }
                }

                if !matched {
                    return Err(format!(
                        "No branches matched in choice <...|...> at position {}",
                        parser.current_token.as_ref().map(|t| t.start).unwrap_or(0)
                    ));
                }
            }
            PatternNode::Group {
                inner,
                separator,
                quantifier,
            } => {
                let mut list_vars: HashMap<String, Vec<SyntaxVar<T>>> = HashMap::new();
                for (n, _) in extract_holes(inner) {
                    list_vars.insert(n.clone(), Vec::new());
                }

                let mut match_count = 0;

                loop {
                    let state = parser.save_state();
                    let mut local_ctx = HashMap::new();
                    let mut local_end_span = *end_span;

                    if eval_pattern_nodes(
                        parser,
                        inner,
                        &mut local_ctx,
                        stop_token_text,
                        &mut local_end_span,
                    )
                    .is_ok()
                    {
                        *end_span = local_end_span;

                        for (k, v) in local_ctx {
                            if let Some(l) = list_vars.get_mut(&k) {
                                match v {
                                    SyntaxVar::List(mut items) => {
                                        l.append(&mut items);
                                    }
                                    _ => {
                                        l.push(v);
                                    }
                                }
                            }
                        }

                        match_count += 1;

                        if *quantifier == Quantifier::ZeroOrOne {
                            break;
                        }

                        if let Some(sep) = separator {
                            let sep_state = parser.save_state();
                            if let Some(ref tk) = parser.current_token {
                                if tk.text == sep {
                                    let t = parser.get_and_bump()?;
                                    *end_span = (t.start + t.len) as u32;
                                } else {
                                    parser.restore_state(sep_state);
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    } else {
                        parser.restore_state(state);
                        break;
                    }
                }

                if *quantifier == Quantifier::OneOrMore && match_count == 0 {
                    return Err(format!(
                        "Expected at least one match for + block at position {}",
                        parser.current_token.as_ref().map(|t| t.start).unwrap_or(0)
                    ));
                }

                for (k, v) in list_vars {
                    ctx_vars.insert(k, SyntaxVar::List(v));
                }
            }
        }
        idx += 1;
    }
    Ok(())
}

fn extract_holes(nodes: &[PatternNode]) -> Vec<(String, HoleKind)> {
    let mut holes = Vec::new();
    for n in nodes {
        match n {
            PatternNode::Hole(name, kind) => holes.push((name.clone(), kind.clone())),
            PatternNode::Group { inner, .. } => holes.extend(extract_holes(inner)),
            PatternNode::Choice(branches) => {
                for b in branches {
                    holes.extend(extract_holes(b));
                }
            }
            _ => {}
        }
    }
    holes
}

fn register_literals(scanner: &mut Scanner, nodes: &[PatternNode]) {
    for node in nodes {
        match node {
            PatternNode::Literal(text) => {
                scanner.add_token(&format!("Token_Auto_{}", text), &escape_for_regex(text));
            }
            PatternNode::Group {
                inner, separator, ..
            } => {
                register_literals(scanner, inner);
                if let Some(sep) = separator {
                    scanner.add_token(&format!("Token_Auto_{}", sep), &escape_for_regex(sep));
                }
            }
            PatternNode::Choice(branches) => {
                for b in branches {
                    register_literals(scanner, b);
                }
            }
            _ => {}
        }
    }
}

fn parse_magic_pattern(pat: &str) -> Vec<PatternNode> {
    let mut nodes = Vec::new();
    let mut chars = pat.char_indices().peekable();

    while let Some(&(_, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == ':' {
            chars.next();
            let mut name = String::new();
            while let Some(&(_, inner)) = chars.peek() {
                if !inner.is_alphanumeric() && inner != '_' {
                    break;
                }
                name.push(inner);
                chars.next();
            }
            nodes.push(PatternNode::Hole(name, HoleKind::Expr));
        } else if c == '@' {
            chars.next();
            let mut name = String::new();
            while let Some(&(_, inner)) = chars.peek() {
                if !inner.is_alphanumeric() && inner != '_' {
                    break;
                }
                name.push(inner);
                chars.next();
            }
            nodes.push(PatternNode::Hole(name, HoleKind::Ident));
        } else if c == '$' {
            chars.next();
            if let Some(&(_, '<')) = chars.peek() {
                chars.next();
                let mut inner_str = String::new();
                let mut depth = 1;
                while let Some(&(_, inner_c)) = chars.peek() {
                    chars.next();
                    if inner_c == '<' {
                        depth += 1;
                    } else if inner_c == '>' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    inner_str.push(inner_c);
                }

                let mut branches = Vec::new();
                let mut current_branch = String::new();
                let mut nest = 0;
                for bc in inner_str.chars() {
                    if bc == '<' || bc == '(' {
                        nest += 1;
                    } else if bc == '>' || bc == ')' {
                        nest -= 1;
                    }

                    if bc == '|' && nest == 0 {
                        branches.push(current_branch.clone());
                        current_branch.clear();
                    } else {
                        current_branch.push(bc);
                    }
                }
                branches.push(current_branch);

                let branch_nodes = branches
                    .into_iter()
                    .map(|s| parse_magic_pattern(&s))
                    .collect();
                nodes.push(PatternNode::Choice(branch_nodes));
            } else if let Some(&(_, '(')) = chars.peek() {
                chars.next();
                let mut inner_str = String::new();
                let mut depth = 1;
                while let Some(&(_, inner_c)) = chars.peek() {
                    chars.next();
                    if inner_c == '(' {
                        depth += 1;
                    } else if inner_c == ')' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    inner_str.push(inner_c);
                }
                let inner_nodes = parse_magic_pattern(&inner_str);

                let mut sep_str = String::new();
                let mut quant = Quantifier::ZeroOrMore;

                while let Some(&(_, sc)) = chars.peek() {
                    if sc.is_whitespace() {
                        chars.next();
                        continue;
                    }
                    if sc == '*' {
                        quant = Quantifier::ZeroOrMore;
                        chars.next();
                        break;
                    } else if sc == '+' {
                        quant = Quantifier::OneOrMore;
                        chars.next();
                        break;
                    } else if sc == '?' {
                        quant = Quantifier::ZeroOrOne;
                        chars.next();
                        break;
                    }
                    sep_str.push(sc);
                    chars.next();
                }

                let sep = if sep_str.is_empty() {
                    None
                } else {
                    Some(sep_str)
                };
                nodes.push(PatternNode::Group {
                    inner: inner_nodes,
                    separator: sep,
                    quantifier: quant,
                });
            } else {
                nodes.push(PatternNode::Literal("$".into()));
            }
        } else {
            let mut lit = String::new();
            while let Some(&(_, inner)) = chars.peek() {
                if inner.is_whitespace() || inner == ':' || inner == '@' || inner == '$' {
                    break;
                }
                if c.is_alphanumeric() != inner.is_alphanumeric() {
                    if c.is_alphanumeric() {
                        break;
                    }
                }
                lit.push(inner);
                chars.next();
            }
            nodes.push(PatternNode::Literal(lit));
        }
    }
    nodes
}

fn escape_for_regex(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if ".*+?()|[]{}\\^$".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}
