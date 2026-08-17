use abyss_nexus::nexus::HirId;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_int(&mut self) -> HirId {
        let span = self.span();
        self.bump();
        let text_value = self.db.tokens.text(self.tk_id(-1));

        match text_value.parse::<u64>() {
            Ok(value) => {
                let id = self.db.hir.alloc_int(self.db.ints.alloc(value));
                self.db.hir_spans.set(id, span);
                self.db.hir_files.set(id, self.file_id);
                id
            }
            Err(_) => {
                self.report_out_of_range_integer_literal(span);
                self.db.hir.alloc_error()
            }
        }
    }

    pub fn parse_float(&mut self) -> HirId {
        let span = self.span();

        self.bump();

        let text_value = self.db.tokens.text(self.tk_id(-1));

        match text_value.parse::<f64>() {
            Ok(value) => {
                let id = self.db.hir.alloc_float(self.db.floats.alloc(value));
                self.db.hir_spans.set(id, span);
                self.db.hir_files.set(id, self.file_id);
                id
            }
            Err(_) => {
                self.report_out_of_range_float_literal(span);
                self.db.hir.alloc_error()
            }
        }
    }
}
