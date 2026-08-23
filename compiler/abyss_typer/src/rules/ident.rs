use abyss_nexus::{arena::ArenaId, nexus::HirId};

use crate::tyck::{TyCtx, Typer};

impl<'a, T: TyCtx> Typer<'a, T> {
    #[inline(always)]
    pub fn synth_ident(&mut self, id: HirId) {
        let db = self.ctx.db_mut();

        let name_id = db.hir.ident_name(id);
        let name = db.interner.get(name_id);

        let slot = db.unify.new_slot(id);

        if let Some((kind, bits)) = parse_builtin_num_type(name) {
            let ty = match kind {
                LitTyKind::Signed => db.types.alloc_int(bits),
                LitTyKind::Unsigned => db.types.alloc_uint(bits),
                LitTyKind::Float => db.types.alloc_float(bits),
                LitTyKind::Bool => db.types.alloc_bool(),
            };

            let ty_id_of_type = db.types.alloc_type();

            db.unify
                .bind_type(&mut db.types, slot, ty_id_of_type)
                .unwrap(); // FIXME

            db.consts.set_type(id, ty);

            return;
        }

        let sym_id = db.hir_to_symbol.get_copy(id);

        if sym_id.is_some() {
            let origin_slot = self.ctx.slot_of(sym_id); // ghat'an 100% slot ghablan barash sakhte shode

            let db = self.ctx.db_mut();

            db.unify.union(&mut db.types, slot, origin_slot).unwrap();
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitTyKind {
    Signed,   // i
    Unsigned, // u
    Float,    // f
    Bool,     // bool
}

pub fn parse_builtin_num_type(name: &str) -> Option<(LitTyKind, u16)> {
    if name == "bool" {
        return Some((LitTyKind::Bool, 0));
    }

    let mut chars = name.chars();

    let kind = match chars.next()? {
        'i' => LitTyKind::Signed,
        'u' => LitTyKind::Unsigned,
        'f' => LitTyKind::Float,
        _ => return None,
    };

    let rest = chars.as_str();

    let bits: u16 = rest.parse().ok()?;

    Some((kind, bits))
}
