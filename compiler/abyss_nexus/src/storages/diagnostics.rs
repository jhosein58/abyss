use abyss_diagnostics::span::Span;

use crate::{
    arena::Arena,
    nexus::{DiagnosticId, FileId, TypeId},
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Help,
    Note,
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    // Parser
    UnexpectedToken,

    // Type Checker
    TypeMismatch { expected: TypeId, found: TypeId },
}

#[derive(Default)]
pub struct DiagnosticStorage {
    // Diagnostics SoA
    pub kinds: Arena<DiagnosticId, DiagnosticKind>,
    pub severities: Arena<DiagnosticId, Severity>,
    pub spans: Arena<DiagnosticId, Span>,
    pub file_ids: Arena<DiagnosticId, FileId>,
    pub help_hints: Arena<DiagnosticId, Option<&'static str>>,

    // Slices for Labels
    pub label_starts: Arena<DiagnosticId, u32>,
    pub label_counts: Arena<DiagnosticId, u16>,

    // Flat Labels SoA Buffer
    pub label_spans: Vec<Span>,
    pub label_messages: Vec<&'static str>,
    pub label_primaries: Vec<bool>,
}
