use abyss_nexus::storages::diagnostics::{DiagnosticKind, DiagnosticMessage};
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn report_unexpected_token(&mut self, expected: TokenKind) {
        let found = self.peek().unwrap_or(TokenKind::Eof);

        self.db.diagnostics.add_label(
            DiagnosticMessage::ExpectedTokenFound,
            self.file_id,
            self.span(),
            true,
        );

        self.db.diagnostics.error(
            DiagnosticKind::UnexpectedToken,
            expected as u32,
            found as u32,
            self.file_id,
            self.span(),
            None,
        );

        self.sync();
    }
}
