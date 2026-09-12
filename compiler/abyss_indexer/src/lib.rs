use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::{Path, PathBuf},
};

use abyss_nexus::{
    arena::ArenaId,
    nexus::{FileId, HirId, NameId, Nexus, TokenId},
    ranges::TokenRange,
};
use abyss_token::kind::TokenKind;

struct PendingImport {
    source_fid: FileId,
    local_name: NameId,
    imported_name: NameId,
    target_fid: FileId,
}

pub struct Indexer;

impl Indexer {
    pub fn index(db: &mut Nexus, root_file_id: FileId, root_path: &str) {
        let root_canonical =
            fs::canonicalize(root_path).unwrap_or_else(|_| PathBuf::from(root_path));

        let mut loaded_files: HashMap<PathBuf, FileId> = HashMap::new();
        loaded_files.insert(root_canonical.clone(), root_file_id);

        let mut queue: VecDeque<(FileId, PathBuf)> = VecDeque::new();
        queue.push_back((root_file_id, root_canonical));

        let mut pending_imports: Vec<PendingImport> = Vec::new();

        while let Some((fid, path)) = queue.pop_front() {
            Self::scan_file_symbols(
                db,
                fid,
                &path,
                &mut loaded_files,
                &mut queue,
                &mut pending_imports,
            );
        }

        for imp in pending_imports {
            let target_sym = *db
                .symbol_index
                .get(&(imp.target_fid, imp.imported_name))
                .unwrap();

            db.symbol_index
                .insert((imp.source_fid, imp.local_name), target_sym);
        }
    }

    fn scan_file_symbols(
        db: &mut Nexus,
        fid: FileId,
        current_file_path: &Path,
        loaded_files: &mut HashMap<PathBuf, FileId>,
        queue: &mut VecDeque<(FileId, PathBuf)>,
        pending_imports: &mut Vec<PendingImport>,
    ) {
        let range = db.file_token_spans.get_copy(fid);
        let mut cursor = range.start.0;
        let end = range.end.0;

        let mut depth: u32 = 0;
        let mut current_symbol: Option<(TokenId, NameId)> = None;

        while cursor < end {
            let tk = db.tokens.kind(TokenId(cursor));

            match tk {
                TokenKind::OBrace | TokenKind::OParen => depth += 1,
                TokenKind::CBrace | TokenKind::CParen => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                TokenKind::Import if depth == 0 => {
                    if let Some((start_tk, name_id)) = current_symbol.take() {
                        let end_tk = TokenId(cursor);
                        let sym_id = db.symbols.alloc(HirId::none());
                        db.symbol_token_range.set(
                            sym_id,
                            TokenRange {
                                start: start_tk,
                                end: end_tk,
                            },
                        );
                        db.symbol_index.insert((fid, name_id), sym_id);
                        db.symbol_files.set(sym_id, fid);
                    }

                    let imported_name_str = db.tokens.text(TokenId(cursor + 1));
                    let imported_name = db.interner.intern(imported_name_str);
                    let local_name = imported_name;

                    let path_str = &db
                        .tokens
                        .text(TokenId(cursor + 3))
                        .trim_matches('"')
                        .to_string();

                    let target_fid = Self::resolve_and_load_file(
                        db,
                        current_file_path,
                        path_str,
                        loaded_files,
                        queue,
                    );

                    pending_imports.push(PendingImport {
                        source_fid: fid,
                        local_name,
                        imported_name,
                        target_fid,
                    });

                    cursor += 4;
                    continue;
                }
                TokenKind::Ident if depth == 0 => {
                    if cursor + 1 < end
                        && db.tokens.kind(TokenId(cursor + 1)) == TokenKind::ColonColon
                    {
                        if cursor + 2 < end
                            && db.tokens.kind(TokenId(cursor + 2)) == TokenKind::Import
                        {
                            if let Some((start_tk, name_id)) = current_symbol.take() {
                                let end_tk = TokenId(cursor);
                                let sym_id = db.symbols.alloc(HirId::none());
                                db.symbol_token_range.set(
                                    sym_id,
                                    TokenRange {
                                        start: start_tk,
                                        end: end_tk,
                                    },
                                );
                                db.symbol_index.insert((fid, name_id), sym_id);
                                db.symbol_files.set(sym_id, fid);
                            }

                            let local_name_str = db.tokens.text(TokenId(cursor));
                            let local_name = db.interner.intern(local_name_str);

                            let imported_name_str = db.tokens.text(TokenId(cursor + 3));
                            let imported_name = db.interner.intern(imported_name_str);

                            let path_str = &db
                                .tokens
                                .text(TokenId(cursor + 5))
                                .trim_matches('"')
                                .to_string();
                            let target_fid = Self::resolve_and_load_file(
                                db,
                                current_file_path,
                                path_str,
                                loaded_files,
                                queue,
                            );

                            pending_imports.push(PendingImport {
                                source_fid: fid,
                                local_name,
                                imported_name,
                                target_fid,
                            });

                            cursor += 6;
                            continue;
                        }

                        if let Some((start_tk, name_id)) = current_symbol.take() {
                            let end_tk = TokenId(cursor);
                            let sym_id = db.symbols.alloc(HirId::none());
                            db.symbol_token_range.set(
                                sym_id,
                                TokenRange {
                                    start: start_tk,
                                    end: end_tk,
                                },
                            );
                            db.symbol_index.insert((fid, name_id), sym_id);
                            db.symbol_files.set(sym_id, fid);
                        }

                        let start_tk = TokenId(cursor);
                        let name = db.tokens.text(start_tk);
                        let name_id = db.interner.intern(name);
                        current_symbol = Some((start_tk, name_id));
                        cursor += 1;
                    }
                }
                _ => {}
            }

            cursor += 1;
        }

        if let Some((start_tk, name_id)) = current_symbol {
            let end_tk = TokenId(end);
            let sym_id = db.symbols.alloc(HirId::none());
            db.symbol_token_range.set(
                sym_id,
                TokenRange {
                    start: start_tk,
                    end: end_tk,
                },
            );
            db.symbol_index.insert((fid, name_id), sym_id);
            db.symbol_files.set(sym_id, fid);
        }
    }

    fn resolve_and_load_file(
        db: &mut Nexus,
        current_file_path: &Path,
        import_path: &str,
        loaded_files: &mut HashMap<PathBuf, FileId>,
        queue: &mut VecDeque<(FileId, PathBuf)>,
    ) -> FileId {
        let target_path = if let Some(parent) = current_file_path.parent() {
            parent.join(import_path)
        } else {
            PathBuf::from(import_path)
        };

        let canonical = fs::canonicalize(&target_path).unwrap_or(target_path);

        if let Some(&fid) = loaded_files.get(&canonical) {
            return fid;
        }

        let content = fs::read_to_string(&canonical).unwrap();
        let fid = db.add_file(&canonical.to_string_lossy(), content);
        db.lex_file(fid);

        loaded_files.insert(canonical.clone(), fid);
        queue.push_back((fid, canonical));

        fid
    }
}
