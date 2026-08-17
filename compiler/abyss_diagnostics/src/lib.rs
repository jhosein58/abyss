use std::fmt::Write;

use abyss_nexus::{
    nexus::{DiagnosticId, FileId, Nexus, TypeId},
    span::Span,
    storages::diagnostics::{DiagnosticKind, DiagnosticMessage, HintMessage, Severity},
};
use abyss_token::kind::TokenKind;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const GRAY: &str = "\x1b[90m";

pub trait DiagnosticFormat {
    fn format_message(&self, nexus: &Nexus, arg0: u32, arg1: u32) -> String;
}

impl DiagnosticFormat for DiagnosticKind {
    fn format_message(&self, _nexus: &Nexus, _arg0: u32, _arg1: u32) -> String {
        match self {
            //parser
            DiagnosticKind::UnexpectedToken => String::from("Unexpected token encountered"),
            DiagnosticKind::InvalidBindingTarget => String::from("Invalid binding target"),
            DiagnosticKind::LiteralOutOfRange => String::from("Literal value out of range"),

            // tyck
            DiagnosticKind::TypeMismatch => String::from("Mismatched types"),
            DiagnosticKind::ExpectedType => String::from("Expected a type, found a value"),
        }
    }
}

impl DiagnosticFormat for DiagnosticMessage {
    fn format_message(&self, nexus: &Nexus, arg0: u32, arg1: u32) -> String {
        match self {
            DiagnosticMessage::TypeMismatchBinOpLhs => {
                format!("this has type '{}'", nexus.types.name(TypeId(arg0)))
            }
            DiagnosticMessage::TypeMismatchBinOpRhs => {
                format!(
                    "expected '{}', found '{}'",
                    nexus.types.name(TypeId(arg0)),
                    nexus.types.name(TypeId(arg1))
                )
            }

            DiagnosticMessage::TypeMismatchDeclExpected => {
                format!("expected due to this type")
            }
            DiagnosticMessage::TypeMismatchDeclFound => {
                format!(
                    "expected '{}', found '{}'",
                    nexus.types.name(TypeId(arg0)),
                    nexus.types.name(TypeId(arg1))
                )
            }
            DiagnosticMessage::ExpectedTypeFoundValue => {
                format!(
                    "expected a type, found value of type '{}'",
                    nexus.types.name(TypeId(arg0))
                )
            }

            DiagnosticMessage::ExpectedTokenFound => {
                format!(
                    "expected token {}, but found {}.",
                    TokenKind::try_from(arg0 as u8).unwrap_or(TokenKind::Unknown),
                    TokenKind::try_from(arg1 as u8).unwrap_or(TokenKind::Unknown)
                )
            }

            DiagnosticMessage::ExpectedIdentifierInBinding => {
                String::from("expected an identifier on the left-hand side of binding operator")
            }
            DiagnosticMessage::IntegerLiteralOverflow => String::from(
                "integer literal is too large to fit in a 64-bit signed integer (`i64`)",
            ),
        }
    }
}

impl DiagnosticFormat for HintMessage {
    fn format_message(&self, _nexus: &Nexus, _arg0: u32, _arg1: u32) -> String {
        match self {
            HintMessage::TypeMismatchBinOp => {
                format!(
                    "binary operator '+', '-', '*', '/' requires both operands to have the same type"
                )
            }

            HintMessage::TypeMismatchDecl => {
                "consider changing the variable type or converting the value to the expected type"
                    .to_string()
            }

            HintMessage::ExpectedTypeHint => {
                "only types (such as 'i32', 'f32', 'bool') are allowed in this position".to_string()
            }

            HintMessage::ParserSyncHint => "this is your problem, not mine.".to_string(),

            HintMessage::BindingPatternNotSupported => String::from(
                "destructuring patterns and complex expressions are not supported here yet; use a simple identifier instead",
            ),

            HintMessage::IntegerLiteralRangeHint => {
                format!("value must be between {} and {}", u64::MIN, u64::MAX)
            }
        }
    }
}

pub struct DiagnosticFormatter<'a> {
    nexus: &'a Nexus,
}

impl<'a> DiagnosticFormatter<'a> {
    pub fn new(nexus: &'a Nexus) -> Self {
        Self { nexus }
    }

    pub fn format_all(&self) -> String {
        let mut buf = String::new();
        for i in 0..self.nexus.diagnostics.len() {
            let id = DiagnosticId(i as u32);
            buf.push_str(&self.format(id));
            buf.push('\n');
        }
        buf
    }

    pub fn format(&self, id: DiagnosticId) -> String {
        let mut buf = String::new();
        let diags = &self.nexus.diagnostics;

        let kind = diags.kinds.get(id);
        let severity = diags.severities.get(id);
        let file_id = *diags.file_ids.get(id);
        let span = *diags.spans.get(id);
        let help = diags.help_hints.get(id);

        let arg0 = *diags.arg0.get(id);
        let arg1 = *diags.arg1.get(id);

        let (sev_color, sev_name) = Self::severity_info(*severity);

        let _ = writeln!(
            buf,
            "{BOLD}{sev_color}{sev_name}{RESET}{BOLD}: {}{RESET}",
            kind.format_message(self.nexus, arg0, arg1)
        );

        let source = self.nexus.sources.get(file_id);

        let (line_num, col_start, col_end, line_text) = Self::resolve_line(source, span);

        let (inline_details, outline_details, max_line) =
            self.categorize_labels(id, file_id, line_num);

        let pad_len = max_line.to_string().len().max(3);
        let empty = " ".repeat(pad_len);
        let margin = format!("{:>width$}", line_num, width = pad_len);

        self.format_file_location(&mut buf, file_id, line_num, col_start, &empty);
        self.format_source_line(&mut buf, &line_text, col_start, col_end, &margin, sev_color);

        if !inline_details.is_empty() {
            self.format_inline_labels(&mut buf, inline_details, &empty, sev_color, arg0, arg1);
        }

        self.format_outline_labels(
            &mut buf,
            outline_details,
            pad_len,
            &empty,
            sev_color,
            arg0,
            arg1,
        );

        if let Some(h) = help {
            let _ = writeln!(buf, "{GRAY}{empty} │{RESET}");
            let _ = writeln!(
                buf,
                "{BOLD}{CYAN}help{RESET}: {}",
                h.format_message(self.nexus, arg0, arg1)
            );
        }

        buf
    }

    fn severity_info(severity: Severity) -> (&'static str, &'static str) {
        match severity {
            Severity::Error => (RED, "error"),
            Severity::Warning => (YELLOW, "warning"),
            Severity::Help => (CYAN, "help"),
            Severity::Note => (GRAY, "note"),
        }
    }

    fn categorize_labels(
        &self,
        id: DiagnosticId,
        file_id: FileId,
        line_num: usize,
    ) -> (
        Vec<(usize, usize, DiagnosticMessage, bool)>,
        Vec<(FileId, usize, usize, usize, String, DiagnosticMessage, bool)>,
        usize,
    ) {
        let diags = &self.nexus.diagnostics;
        let label_start = *diags.label_starts.get(id) as usize;
        let label_count = *diags.label_counts.get(id) as usize;

        let mut inline = Vec::new();
        let mut outline = Vec::new();
        let mut max_line = line_num;

        for i in 0..label_count {
            let idx = label_start + i;
            let l_file = diags.label_file_ids[idx];
            let l_span = diags.label_spans[idx];
            let l_msg = diags.label_messages[idx];
            let l_is_primary = diags.label_primaries[idx];

            let l_source = self.nexus.sources.get(l_file);
            let (l_line, l_cstart, l_cend, l_text) = Self::resolve_line(l_source, l_span);

            if l_file == file_id && l_line == line_num {
                inline.push((
                    l_cstart.saturating_sub(1),
                    l_cend.saturating_sub(1),
                    l_msg,
                    l_is_primary,
                ));
            } else {
                max_line = max_line.max(l_line);
                outline.push((
                    l_file,
                    l_line,
                    l_cstart,
                    l_cend,
                    l_text,
                    l_msg,
                    l_is_primary,
                ));
            }
        }

        (inline, outline, max_line)
    }

    fn format_file_location(
        &self,
        buf: &mut String,
        file_id: FileId,
        line_num: usize,
        col_start: usize,
        empty: &str,
    ) {
        let _ = writeln!(
            buf,
            "{GRAY}{empty}─▶ {}:{}:{}{RESET}",
            self.nexus
                .interner
                .get(*self.nexus.file_to_name.get(file_id)),
            line_num,
            col_start
        );
        let _ = writeln!(buf, "{GRAY}{empty} │{RESET}");
    }

    fn format_source_line(
        &self,
        buf: &mut String,
        line_text: &str,
        col_start: usize,
        col_end: usize,
        margin: &str,
        sev_color: &str,
    ) {
        let _ = write!(buf, "{GRAY}{margin} │ {RESET}");

        let safe_start = col_start.saturating_sub(1).min(line_text.len());
        let safe_end = col_end.saturating_sub(1).min(line_text.len());

        let before = &line_text[..safe_start];
        let problem = &line_text[safe_start..safe_end];
        let after = &line_text[safe_end..];

        let _ = write!(buf, "{GRAY}{before}{RESET}");
        let _ = write!(buf, "{sev_color}{problem}{RESET}");
        let _ = writeln!(buf, "{GRAY}{after}{RESET}");
    }

    fn format_inline_labels(
        &self,
        buf: &mut String,
        mut inline: Vec<(usize, usize, DiagnosticMessage, bool)>,
        empty: &str,
        sev_color: &str,
        arg0: u32,
        arg1: u32,
    ) {
        inline.sort_by_key(|l| (l.0, !l.3));

        let max_end = inline.iter().map(|l| l.1).max().unwrap_or(0);
        let mut caret_line = vec![(' ', ""); max_end];

        let mut sorted_labels = inline.clone();
        sorted_labels.sort_by_key(|l| (l.3, l.0));

        for (start, end, _, is_primary) in &sorted_labels {
            let s = *start;
            let e = (*end).max(s + 1);
            let ch = if *is_primary { '^' } else { '-' };
            let color = if *is_primary { sev_color } else { BLUE };

            if e > caret_line.len() {
                caret_line.resize(e, (' ', ""));
            }
            for i in s..e {
                caret_line[i] = (ch, color);
            }
        }

        let mut caret_str = String::new();
        let mut curr_color = "";
        for (ch, color) in caret_line {
            if ch == ' ' {
                if !curr_color.is_empty() {
                    caret_str.push_str(RESET);
                    curr_color = "";
                }
                caret_str.push(' ');
            } else {
                if color != curr_color {
                    caret_str.push_str(color);
                    curr_color = color;
                }
                caret_str.push(ch);
            }
        }
        if !curr_color.is_empty() {
            caret_str.push_str(RESET);
        }

        let _ = writeln!(buf, "{GRAY}{empty} │ {RESET}{caret_str}");

        let mut active_starts: Vec<(usize, &str)> = inline
            .iter()
            .map(|l| (l.0, if l.3 { sev_color } else { BLUE }))
            .collect();

        for (start, _end, msg, is_primary) in inline.into_iter().rev() {
            let color = if is_primary { sev_color } else { BLUE };

            if let Some(pos) = active_starts.iter().rposition(|&(x, _)| x == start) {
                active_starts.remove(pos);
            }

            let mut msg_line_str = String::new();
            let mut curr_col = "";

            for i in 0..start {
                if let Some(&(_, c)) = active_starts.iter().find(|&&(s, _)| s == i) {
                    if curr_col != c {
                        msg_line_str.push_str(c);
                        curr_col = c;
                    }
                    msg_line_str.push('│');
                } else {
                    if !curr_col.is_empty() {
                        msg_line_str.push_str(RESET);
                        curr_col = "";
                    }
                    msg_line_str.push(' ');
                }
            }
            if !curr_col.is_empty() {
                msg_line_str.push_str(RESET);
            }

            let has_more = active_starts.iter().any(|&(s, _)| s == start);
            let branch = if has_more { "├──" } else { "╰──" };

            let formatted_msg = msg.format_message(self.nexus, arg0, arg1);

            let _ = writeln!(
                buf,
                "{GRAY}{empty} │ {RESET}{msg_line_str}{color}{} {}{RESET}",
                branch, formatted_msg
            );
        }
    }

    fn format_outline_labels(
        &self,
        buf: &mut String,
        outline: Vec<(FileId, usize, usize, usize, String, DiagnosticMessage, bool)>,
        pad_len: usize,
        empty: &str,
        sev_color: &str,
        arg0: u32,
        arg1: u32,
    ) {
        for (l_file, l_line, l_cstart, l_cend, l_text, l_msg, l_is_primary) in outline {
            let l_margin = format!("{:>width$}", l_line, width = pad_len);
            let color = if l_is_primary { sev_color } else { BLUE };
            let ch = if l_is_primary { '^' } else { '-' };

            let _ = writeln!(buf, "{GRAY}{empty} │{RESET}");
            self.format_file_location(buf, l_file, l_line, l_cstart, empty);

            let _ = write!(buf, "{GRAY}{l_margin} │ {RESET}");

            let ll = l_text.len();
            let ss = l_cstart.saturating_sub(1).min(ll);
            let se = l_cend.saturating_sub(1).min(ll);

            let _ = write!(buf, "{GRAY}{}{RESET}", &l_text[..ss]);
            let _ = write!(buf, "{color}{}{RESET}", &l_text[ss..se]);
            let _ = writeln!(buf, "{GRAY}{}{RESET}", &l_text[se..]);

            let max_len = se.max(ss + 1);
            let mut caret_line = vec![' '; max_len];
            for i in ss..se.max(ss + 1) {
                if i < caret_line.len() {
                    caret_line[i] = ch;
                } else {
                    caret_line.push(ch);
                }
            }
            let caret_str: String = caret_line.into_iter().collect();
            let _ = writeln!(buf, "{GRAY}{empty} │ {RESET}{color}{caret_str}{RESET}");

            let msg_str = " ".repeat(ss);
            let formatted_msg = l_msg.format_message(self.nexus, arg0, arg1);

            let _ = writeln!(
                buf,
                "{GRAY}{empty} │ {RESET}{color}{msg_str}╰── {}{RESET}",
                formatted_msg
            );
        }
    }

    fn resolve_line(source: &str, span: Span) -> (usize, usize, usize, String) {
        let start = span.start as usize;
        let end = span.end as usize;
        let bytes = source.as_bytes();

        let mut line_num = 1;
        let mut line_start = 0;

        for i in 0..start.min(bytes.len()) {
            if bytes[i] == b'\n' {
                line_num += 1;
                line_start = i + 1;
            }
        }

        let mut line_end = line_start;
        while line_end < bytes.len() && bytes[line_end] != b'\n' {
            line_end += 1;
        }

        let line_text = String::from_utf8_lossy(&bytes[line_start..line_end]).into_owned();

        let col_start = start.saturating_sub(line_start) + 1;
        let col_end = end.saturating_sub(line_start) + 1;

        (line_num, col_start, col_end, line_text)
    }
}
