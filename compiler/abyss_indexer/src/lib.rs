use std::fs;

use abyss_nexus::{
    arena::ArenaId,
    nexus::{FileId, HirId, NameId, Nexus, TokenId},
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
                TokenKind::Import if depth == 0 => {
                    if let Some((start_tk, name_id)) = current_symbol.take() {
                        let end_tk = TokenId(self.cursor);
                        let sym_id = self.db.symbols.alloc(HirId::none());
                        self.db.symbol_token_range.set(
                            sym_id,
                            TokenRange {
                                start: start_tk,
                                end: end_tk,
                            },
                        );
                        self.db.symbol_index.insert((self.file_id, name_id), sym_id);
                        self.db.symbol_files.set(sym_id, self.file_id);
                    }

                    let imported_name = self.db.tokens.text(TokenId(self.cursor + 1)).to_string();
                    let imported_path = self
                        .db
                        .tokens
                        .text(TokenId(self.cursor + 3))
                        .trim_matches('"')
                        .to_string();

                    let content = fs::read_to_string(&imported_path).unwrap();
                    let fid = self.db.add_file(&imported_path, content);
                    self.db.lex_file(fid);
                    Indexer::index(self.db, fid);

                    let name_id = self.db.interner.intern(&imported_name);
                    let target_sym = *self.db.symbol_index.get(&(fid, name_id)).unwrap();
                    self.db
                        .symbol_index
                        .insert((self.file_id, name_id), target_sym);

                    self.cursor += 4;
                    continue;
                }
                TokenKind::Ident if depth == 0 => {
                    if self.cursor + 1 < self.end
                        && self.db.tokens.kind(TokenId(self.cursor + 1)) == TokenKind::ColonColon
                    {
                        if self.cursor + 2 < self.end
                            && self.db.tokens.kind(TokenId(self.cursor + 2)) == TokenKind::Import
                        {
                            if let Some((start_tk, name_id)) = current_symbol.take() {
                                let end_tk = TokenId(self.cursor);
                                let sym_id = self.db.symbols.alloc(HirId::none());
                                self.db.symbol_token_range.set(
                                    sym_id,
                                    TokenRange {
                                        start: start_tk,
                                        end: end_tk,
                                    },
                                );
                                self.db.symbol_index.insert((self.file_id, name_id), sym_id);
                                self.db.symbol_files.set(sym_id, self.file_id);
                            }

                            let local_name = self.db.tokens.text(TokenId(self.cursor)).to_string();
                            let local_name_id = self.db.interner.intern(&local_name);

                            let imported_name =
                                self.db.tokens.text(TokenId(self.cursor + 3)).to_string();
                            let imported_path = self
                                .db
                                .tokens
                                .text(TokenId(self.cursor + 5))
                                .trim_matches('"')
                                .to_string();

                            let content = fs::read_to_string(&imported_path).unwrap();
                            let fid = self.db.add_file(&imported_path, content);
                            self.db.lex_file(fid);
                            Indexer::index(self.db, fid);

                            let target_name_id = self.db.interner.intern(&imported_name);
                            let target_sym =
                                *self.db.symbol_index.get(&(fid, target_name_id)).unwrap();
                            self.db
                                .symbol_index
                                .insert((self.file_id, local_name_id), target_sym);

                            self.cursor += 6;
                            continue;
                        }

                        if let Some((start_tk, name_id)) = current_symbol {
                            let end_tk = TokenId(self.cursor);
                            let sym_id = self.db.symbols.alloc(HirId::none());

                            self.db.symbol_token_range.set(
                                sym_id,
                                TokenRange {
                                    start: start_tk,
                                    end: end_tk,
                                },
                            );

                            self.db.symbol_index.insert((self.file_id, name_id), sym_id);
                            self.db.symbol_files.set(sym_id, self.file_id);
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
            let end_tk = TokenId(self.end);
            let sym_id = self.db.symbols.alloc(HirId::none());

            self.db.symbol_token_range.set(
                sym_id,
                TokenRange {
                    start: start_tk,
                    end: end_tk,
                },
            );

            self.db.symbol_index.insert((self.file_id, name_id), sym_id);
            self.db.symbol_files.set(sym_id, self.file_id);
        }
    }
}
