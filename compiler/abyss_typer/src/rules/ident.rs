use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let name_id = db.hir.ident_name(id);
    let name = db.interner.get(name_id);

    if let Some((kind, bits)) = parse_builtin_num_type(name) {
        let ty = match kind {
            NumTypeKind::Signed => db.types.alloc_int(bits),
            NumTypeKind::Unsigned => db.types.alloc_uint(bits),
            NumTypeKind::Float => db.types.alloc_float(bits),
        };

        db.hir_to_type.set(id, ty);
        return;
    }

    let sym_id = db.hir_to_symbol.get_copy(id);

    if sym_id.is_some() {
        let origin_hir_id = db.symbols.get_copy(sym_id);
        let ty = db.hir_to_type.get_copy(origin_hir_id);
        db.hir_to_type.set(id, ty);
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumTypeKind {
    Signed,   // i
    Unsigned, // u
    Float,    // f
}

pub fn parse_builtin_num_type(name: &str) -> Option<(NumTypeKind, u16)> {
    let mut chars = name.chars();

    let kind = match chars.next()? {
        'i' => NumTypeKind::Signed,
        'u' => NumTypeKind::Unsigned,
        'f' => NumTypeKind::Float,
        _ => return None,
    };

    let rest = chars.as_str();

    let bits: u16 = rest.parse().ok()?;

    Some((kind, bits))
}
