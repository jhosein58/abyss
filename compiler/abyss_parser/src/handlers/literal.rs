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
                self.report_int_overflow(span);
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
                if value == f64::INFINITY {
                    self.report_float_overflow(span);
                    return self.db.hir.alloc_error();
                }

                let id = self.db.hir.alloc_float(self.db.floats.alloc(value));
                self.db.hir_spans.set(id, span);
                self.db.hir_files.set(id, self.file_id);

                id
            }

            Err(_) => {
                self.report_float_overflow(span);
                self.db.hir.alloc_error()
            }
        }
    }

    #[inline(always)]
    pub fn parse_true(&mut self) -> HirId {
        self.bump();
        self.db.hir.alloc_true()
    }

    #[inline(always)]
    pub fn parse_false(&mut self) -> HirId {
        self.bump();
        self.db.hir.alloc_false()
    }

    #[inline(always)]
    pub fn parse_str(&mut self) -> HirId {
        let span = self.span();
        let text_value = self
            .db
            .tokens
            .text(self.tk_id(0))
            .trim_matches('"')
            .to_string();

        self.bump();

        let mut str_bytes = text_value.as_bytes().to_vec();
        str_bytes.push(0);

        let mut allocated_lits = Vec::with_capacity(str_bytes.len());

        for b in str_bytes {
            let lit_id = self.db.hir.alloc_int(self.db.ints.alloc(b as u64));
            self.db.hir_spans.set_safe(lit_id, span);
            self.db.hir_files.set_safe(lit_id, self.file_id);
            allocated_lits.push(lit_id.0);
        }

        let zero_lit_id = self.db.hir.alloc_int(self.db.ints.alloc(0));
        self.db.hir_spans.set_safe(zero_lit_id, span);
        self.db.hir_files.set_safe(zero_lit_id, self.file_id);

        let list_id = self.db.add_list_flat(&allocated_lits);

        let array_id = self.db.hir.alloc_array_init(list_id);
        self.db.hir_spans.set_safe(array_id, span);
        self.db.hir_files.set_safe(array_id, self.file_id);

        let indexed_arr_id = self.db.hir.alloc_index(array_id, zero_lit_id);
        self.db.hir_spans.set_safe(indexed_arr_id, span);
        self.db.hir_files.set_safe(indexed_arr_id, self.file_id);

        let addrof_id = self.db.hir.alloc_addrof(indexed_arr_id);
        self.db.hir_spans.set_safe(addrof_id, span);
        self.db.hir_files.set_safe(addrof_id, self.file_id);

        addrof_id
    }
}
