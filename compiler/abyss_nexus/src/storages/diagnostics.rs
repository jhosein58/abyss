use crate::{
    arena::{Arena, SideTable},
    nexus::{DiagnosticId, FileId},
    span::Span,
};

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Help,
    Note,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticMessage {
    // Parser
    ExpectedTokenFound,
    ExpectedIdentifierInBinding,
    IntegerLiteralOverflow,
    FloatLiteralOverflow,

    // Type Checker

    // Type Checker - Binary Operations
    TypeMismatchBinOpLhs,
    TypeMismatchBinOpRhs,

    // Type Checker - Variable Declaration
    TypeMismatchDeclExpected,
    TypeMismatchDeclFound,

    // Type Checker - Type Position
    ExpectedTypeFoundValue,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum HintMessage {
    // Parser
    ParserSyncHint,
    BindingPatternNotSupported,
    IntegerLiteralRangeHint,
    FloatLiteralRangeHint,
    ExpectedExpressionHint,
    UnexpectedEofHint,

    // Type Checker
    TypeMismatchBinOp,
    TypeMismatchDecl,
    ExpectedTypeHint,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    // Parser
    UnexpectedToken,
    InvalidBindingTarget,
    LiteralOutOfRange,
    ExpectedExpression,
    UnexpectedEof,

    // Type Checker
    TypeMismatch,
    ExpectedType,
}

pub struct DiagnosticStorage {
    /// Primary Diagnostic Arena
    pub kinds: Arena<DiagnosticId, DiagnosticKind>,

    // SideTables
    pub arg0: SideTable<DiagnosticId, u32>,
    pub arg1: SideTable<DiagnosticId, u32>,
    pub severities: SideTable<DiagnosticId, Severity>,
    pub spans: SideTable<DiagnosticId, Span>,
    pub file_ids: SideTable<DiagnosticId, FileId>,
    pub help_hints: SideTable<DiagnosticId, Option<HintMessage>>,

    // Slices for Labels
    pub label_starts: SideTable<DiagnosticId, u32>,
    pub label_counts: SideTable<DiagnosticId, u16>,

    // Flat Labels SoA Buffer
    pub label_file_ids: Vec<FileId>,
    pub label_spans: Vec<Span>,
    pub label_messages: Vec<DiagnosticMessage>,
    pub label_primaries: Vec<bool>,

    // state
    offset: u32,
    len: u16,
}

impl Default for DiagnosticStorage {
    fn default() -> Self {
        Self::with_capacity(64)
    }
}

impl DiagnosticStorage {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            kinds: Arena::with_capacity(capacity),
            arg0: SideTable::with_capacity(capacity),
            arg1: SideTable::with_capacity(capacity),
            severities: SideTable::with_capacity(capacity),
            spans: SideTable::with_capacity(capacity),
            file_ids: SideTable::with_capacity(capacity),
            help_hints: SideTable::with_capacity(capacity),
            label_starts: SideTable::with_capacity(capacity),
            label_counts: SideTable::with_capacity(capacity),
            label_file_ids: Vec::with_capacity(capacity),
            label_spans: Vec::with_capacity(capacity),
            label_messages: Vec::with_capacity(capacity),
            label_primaries: Vec::with_capacity(capacity),
            offset: 0,
            len: 0,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    #[inline]
    pub fn add_label(
        &mut self,
        message: DiagnosticMessage,
        file_id: FileId,
        span: Span,
        is_primary: bool,
    ) {
        if self.len == 0 {
            self.offset = self.label_messages.len() as u32;
        }
        self.label_messages.push(message);
        self.label_file_ids.push(file_id);
        self.label_spans.push(span);
        self.label_primaries.push(is_primary);
        self.len += 1;
    }

    pub fn emit(
        &mut self,
        kind: DiagnosticKind,
        severity: Severity,
        arg0: u32,
        arg1: u32,
        file_id: FileId,
        span: Span,
        help_hint: Option<HintMessage>,
    ) -> DiagnosticId {
        let id = self.kinds.alloc(kind);
        let len = self.kinds.len();

        self.arg0.grow_to(len);
        self.arg1.grow_to(len);
        self.severities.grow_to(len);
        self.spans.grow_to(len);
        self.file_ids.grow_to(len);
        self.help_hints.grow_to(len);
        self.label_starts.grow_to(len);
        self.label_counts.grow_to(len);

        self.arg0.set(id, arg0);
        self.arg1.set(id, arg1);
        self.severities.set(id, severity);
        self.file_ids.set(id, file_id);
        self.spans.set(id, span);
        self.help_hints.set(id, help_hint);

        self.label_starts.set(id, self.offset);
        self.label_counts.set(id, self.len);

        // Reset state
        self.len = 0;
        self.offset = 0;

        id
    }

    #[inline]
    pub fn error(
        &mut self,
        kind: DiagnosticKind,
        arg0: u32,
        arg1: u32,
        file_id: FileId,
        span: Span,
        help_hint: Option<HintMessage>,
    ) -> DiagnosticId {
        self.emit(kind, Severity::Error, arg0, arg1, file_id, span, help_hint)
    }

    #[inline]
    pub fn warning(
        &mut self,
        kind: DiagnosticKind,
        arg0: u32,
        arg1: u32,
        file_id: FileId,
        span: Span,
        help_hint: Option<HintMessage>,
    ) -> DiagnosticId {
        self.emit(
            kind,
            Severity::Warning,
            arg0,
            arg1,
            file_id,
            span,
            help_hint,
        )
    }
}
