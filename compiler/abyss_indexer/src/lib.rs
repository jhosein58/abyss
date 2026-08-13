use abyss_nexus::{
    nexus::{FileId, NameId, Nexus, TokenId},
    ranges::TokenRange,
};
use abyss_token::kind::TokenKind;

pub struct Indexer<'a> {
    db: &'a mut Nexus,
    cursor: u32,
    end: u32,
    file_id: FileId,
}

impl<'a> Indexer<'a> {
    #[inline(always)]
    pub fn index(db: &'a mut Nexus, file_id: FileId) {
        let range = db.file_token_spans.get_copy(file_id);
        let mut scanner = Indexer {
            db,
            cursor: range.start.0,
            end: range.end.0,
            file_id,
        };

        scanner.scan_symbols();
    }

    #[inline(always)]
    fn scan_symbols(&mut self) {
        let mut depth: u32 = 0;
        let mut current_symbol: Option<(TokenId, NameId)> = None;

        while self.cursor < self.end {
            let tk = self.db.tokens.kind(TokenId(self.cursor));

            match tk {
                TokenKind::OBrace | TokenKind::OParen => depth += 1,
                TokenKind::CBrace | TokenKind::CParen => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                TokenKind::Ident if depth == 0 => {
                    if self.cursor + 1 < self.end
                        && self.db.tokens.kind(TokenId(self.cursor + 1)) == TokenKind::ColonColon
                    {
                        if let Some((start_tk, name_id)) = current_symbol {
                            let end_tk = TokenId(self.cursor - 1);
                            self.db.symbol_index.insert(
                                (self.file_id, name_id),
                                TokenRange {
                                    start: start_tk,
                                    end: end_tk,
                                },
                            );
                        }

                        let start_tk = TokenId(self.cursor);
                        let name = self.db.tokens.text(start_tk);
                        let name_id = self.db.interner.intern(name);

                        current_symbol = Some((start_tk, name_id));
                        self.cursor += 1;
                    }
                }
                _ => {}
            }

            self.cursor += 1;
        }

        if let Some((start_tk, name_id)) = current_symbol {
            let end_tk = TokenId(self.end.saturating_sub(1));
            self.db.symbol_index.insert(
                (self.file_id, name_id),
                TokenRange {
                    start: start_tk,
                    end: end_tk,
                },
            );
        }
    }
}
