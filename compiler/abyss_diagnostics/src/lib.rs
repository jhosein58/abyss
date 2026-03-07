use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct Span {
    pub file_id: u16,
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn empty() -> Self {
        Self {
            file_id: 0,
            start: 0,
            end: 0,
        }
    }

    pub fn merge(self, other: Span) -> Span {
        debug_assert_eq!(
            self.file_id, other.file_id,
            "Cannot merge spans from different files"
        );
        Span {
            file_id: self.file_id,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct SourceMap {
    line_starts: Vec<usize>,
}

impl SourceMap {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        line_starts.extend(
            source
                .char_indices()
                .filter(|&(_, c)| c == '\n')
                .map(|(i, _)| i + 1),
        );
        Self { line_starts }
    }

    pub fn find_position(&self, offset: usize, source: &str) -> Option<Position> {
        let line_index = self.line_starts.partition_point(|&start| start <= offset) - 1;
        let line_start_offset = *self.line_starts.get(line_index)?;
        let line = line_index + 1;
        let line_content_up_to_offset = source.get(line_start_offset..offset)?;
        let column = line_content_up_to_offset.chars().count() + 1;
        Some(Position { line, column })
    }

    pub fn get_line_bounds(&self, line: usize, source_len: usize) -> (usize, usize) {
        let line_index = line - 1;
        let start = self.line_starts[line_index];

        let end = if line_index + 1 < self.line_starts.len() {
            self.line_starts[line_index + 1] - 1
        } else {
            source_len
        };

        (start, end)
    }
}

pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[92m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const CYAN: &str = "\x1b[36m";
    pub const BOLD: &str = "\x1b[1m";
    pub const GRAY: &str = "\x1b[90m";
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Error,
    Warning,
    Note,
}

impl Level {
    fn color(&self) -> &'static str {
        match self {
            Level::Error => colors::RED,
            Level::Warning => colors::YELLOW,
            Level::Note => colors::CYAN,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Note => "note",
        }
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub content: String,
    pub map: SourceMap,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}

pub struct DiagnosticEngine {
    files: HashMap<u16, SourceFile>,
    diagnostics: Vec<Diagnostic>,
}

impl Default for DiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn add_source(&mut self, file_id: u16, filename: String, content: String) {
        let map = SourceMap::new(&content);
        self.files.insert(
            file_id,
            SourceFile {
                name: filename,
                content,
                map,
            },
        );
    }

    pub fn report_error(&mut self, span: Span, message: String) {
        self.diagnostics.push(Diagnostic {
            level: Level::Error,
            message,
            span,
            hint: None,
        });
    }

    pub fn report_error_with_hint(&mut self, span: Span, message: String, hint: String) {
        self.diagnostics.push(Diagnostic {
            level: Level::Error,
            message,
            span,
            hint: Some(hint),
        });
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.level, Level::Error))
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        for diag in &self.diagnostics {
            let file_info = self.files.get(&diag.span.file_id);

            writeln!(
                &mut out,
                "{}{}{}: {}{}",
                diag.level.color(),
                colors::BOLD,
                diag.level.name(),
                colors::RESET,
                diag.message
            )
            .unwrap();

            if let Some(file) = file_info {
                if let Some(pos) = file
                    .map
                    .find_position(diag.span.start as usize, &file.content)
                {
                    writeln!(
                        &mut out,
                        "  {}--> {}:{}:{}{}",
                        colors::GRAY,
                        file.name,
                        pos.line,
                        pos.column,
                        colors::RESET
                    )
                    .unwrap();

                    let (line_start, mut line_end) =
                        file.map.get_line_bounds(pos.line, file.content.len());

                    if line_end > 0 && file.content.as_bytes().get(line_end - 1) == Some(&b'\r') {
                        line_end -= 1;
                    }

                    let start_idx = (diag.span.start as usize).max(line_start).min(line_end);
                    let end_idx = (diag.span.end as usize).max(start_idx).min(line_end);

                    let before = &file.content[line_start..start_idx];
                    let error_part = &file.content[start_idx..end_idx];
                    let after = &file.content[end_idx..line_end];

                    let line_str = pos.line.to_string();
                    let margin = " ".repeat(line_str.len());

                    writeln!(&mut out, "   {} {}|{}", margin, colors::GRAY, colors::RESET).unwrap();

                    writeln!(
                        &mut out,
                        "   {} {}|{} {}{}{}{}{}{}{}{}{}{}",
                        colors::GRAY,
                        line_str,
                        colors::RESET,
                        colors::GRAY,
                        before,
                        colors::RESET,
                        colors::RED,
                        colors::BOLD,
                        error_part,
                        colors::RESET,
                        colors::GRAY,
                        after,
                        colors::RESET
                    )
                    .unwrap();

                    writeln!(&mut out, "   {} {}|{}", margin, colors::GRAY, colors::RESET).unwrap();
                }
            }

            if let Some(hint) = &diag.hint {
                let margin = if let Some(file) = file_info {
                    if let Some(pos) = file
                        .map
                        .find_position(diag.span.start as usize, &file.content)
                    {
                        " ".repeat(pos.line.to_string().len())
                    } else {
                        " ".to_string()
                    }
                } else {
                    " ".to_string()
                };

                writeln!(
                    &mut out,
                    "   {} {}={} {}{}{}",
                    margin,
                    colors::GRAY,
                    colors::GREEN,
                    colors::BOLD,
                    hint,
                    colors::RESET
                )
                .unwrap();
            }

            writeln!(&mut out).unwrap();
        }

        out
    }
}
