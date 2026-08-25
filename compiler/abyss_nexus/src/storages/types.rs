use std::{collections::HashMap, hash::Hash};

use abyss_types::{TyKind, TyStore};

use crate::{
    arena::ArenaId,
    nexus::{NameId, TypeId},
};

impl TypeId {
    pub const UNKNOWN: TypeId = TypeId(0);
    pub const UNTYPED_INT: TypeId = TypeId(1);
    pub const UNTYPED_FLOAT: TypeId = TypeId(2);
    pub const BOOL: TypeId = TypeId(3);
    pub const TYPE: TypeId = TypeId(4);
    pub const UNIT: TypeId = TypeId(5);
    pub const NEVER: TypeId = TypeId(6);
    pub const ERROR: TypeId = TypeId(7);

    pub const BUILTIN_COUNT: usize = 8;
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)] // FIXME: impl trait Copy
pub enum TypeKey {
    Int(u16), // bit width
    UInt(u16),
    Float(u16),
    Ptr(TypeId),
    Func(Box<[TypeId]>, TypeId, bool), // Key(params, return, is_extern), PERF: Box ro hazf kon
    Struct(Box<[(NameId, TypeId)]>),   // FIXME: remove allocation
}

pub struct TypeStorage {
    store: TyStore,
    interned: HashMap<TypeKey, TypeId>, // PERF: hash-map inja mitone sari tar beshe ya kolan hazf beshe
                                        // IDEA: use a pre-allocated table to lockup primitive types
}

impl Default for TypeStorage {
    fn default() -> Self {
        let mut store = TyStore::default();
        store.reserve(TypeId::BUILTIN_COUNT);

        let id0 = store.push(TyKind::Unknown, 0);
        let id1 = store.push(TyKind::UntypedInt, 0);
        let id2 = store.push(TyKind::UntypedFloat, 0);
        let id3 = store.push(TyKind::Bool, 0);
        let id4 = store.push(TyKind::Type, 0);
        let id5 = store.push(TyKind::Unit, 0);
        let id6 = store.push(TyKind::Never, 0);
        let id7 = store.push(TyKind::Error, 0);

        debug_assert_eq!(id0, TypeId::UNKNOWN.0 as usize);
        debug_assert_eq!(id1, TypeId::UNTYPED_INT.0 as usize);
        debug_assert_eq!(id2, TypeId::UNTYPED_FLOAT.0 as usize);
        debug_assert_eq!(id3, TypeId::BOOL.0 as usize);
        debug_assert_eq!(id4, TypeId::TYPE.0 as usize);
        debug_assert_eq!(id5, TypeId::UNIT.0 as usize);
        debug_assert_eq!(id6, TypeId::NEVER.0 as usize);
        debug_assert_eq!(id7, TypeId::ERROR.0 as usize);

        Self {
            store,
            interned: HashMap::default(),
        }
    }
}

impl TypeStorage {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.store.len()
    }

    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.store.reserve(additional);
    }

    #[inline(always)]
    pub fn kind(&self, idx: TypeId) -> TyKind {
        if idx.is_none() {
            return TyKind::Unknown;
        }

        self.store.kinds[idx.0 as usize]
    }

    #[inline(always)]
    pub fn payload(&self, idx: TypeId) -> u32 {
        self.store.payload[idx.0 as usize]
    }

    #[inline(always)]
    pub fn unify_types(&mut self, a: TypeId, b: TypeId) -> Result<TypeId, (TypeId, TypeId)> {
        if a == b {
            return Ok(a);
        }

        let kind_a = self.kind(a);
        let kind_b = self.kind(b);

        if kind_a == TyKind::Never {
            return Ok(b);
        }
        if kind_b == TyKind::Never {
            return Ok(a);
        }

        if kind_a == TyKind::Unknown {
            return Ok(b);
        }
        if kind_b == TyKind::Unknown {
            return Ok(a);
        }

        match (kind_a, kind_b) {
            (
                TyKind::UntypedInt,
                TyKind::Int | TyKind::UInt | TyKind::Float | TyKind::UntypedFloat,
            ) => Ok(b),
            (
                TyKind::Int | TyKind::UInt | TyKind::Float | TyKind::UntypedFloat,
                TyKind::UntypedInt,
            ) => Ok(a),

            (TyKind::UntypedFloat, TyKind::Float) => Ok(b),
            (TyKind::Float, TyKind::UntypedFloat) => Ok(a),

            (TyKind::Ptr, TyKind::Ptr) => {
                let inner_a = TypeId(self.payload(a));
                let inner_b = TypeId(self.payload(b));
                let unified_inner = self.unify_types(inner_a, inner_b)?;
                Ok(self.alloc_ptr(unified_inner))
            }

            // TODO: Func type
            _ => Err((a, b)),
        }
    }

    pub fn name(&self, idx: TypeId) -> String {
        if idx.is_none() {
            return "".to_string();
        }

        match self.kind(idx) {
            TyKind::Unknown => "Unknown".to_string(),

            TyKind::UntypedInt => format!("UtInt"),
            TyKind::UntypedFloat => format!("UtFloat"),
            TyKind::Int => format!("i{}", self.payload(idx)),
            TyKind::UInt => format!("u{}", self.payload(idx)),
            TyKind::Float => format!("f{}", self.payload(idx)),
            TyKind::Bool => format!("bool"),
            TyKind::Ptr => format!("&{}", self.name(TypeId(self.payload(idx)))),
            TyKind::Type => format!("Type"),
            TyKind::Unit => format!("unit"),
            TyKind::Never => format!("!"),

            TyKind::Func => format!(
                "fn({}) {}",
                self.func_params(idx)
                    .iter()
                    .map(|v| self.name(*v))
                    .collect::<Vec<String>>()
                    .join(", "),
                self.name(self.func_return(idx))
            ),

            TyKind::Struct => todo!(),

            TyKind::Error => format!("Err!"),
        }
    }

    #[inline(always)]
    fn get_or_insert(&mut self, key: TypeKey, kind: TyKind, payload: u32) -> TypeId {
        *self
            .interned
            .entry(key)
            .or_insert_with(|| TypeId(self.store.push(kind, payload) as u32))
    }

    #[inline(always)]
    pub fn alloc_unknown(&self) -> TypeId {
        TypeId::UNKNOWN
    }

    #[inline(always)]
    pub fn alloc_untyped_int(&self) -> TypeId {
        TypeId::UNTYPED_INT
    }

    #[inline(always)]
    pub fn alloc_untyped_float(&self) -> TypeId {
        TypeId::UNTYPED_FLOAT
    }

    #[inline(always)]
    pub fn alloc_bool(&self) -> TypeId {
        TypeId::BOOL
    }

    #[inline(always)]
    pub fn alloc_error(&self) -> TypeId {
        TypeId::ERROR
    }

    #[inline(always)]
    pub fn alloc_type(&self) -> TypeId {
        TypeId::TYPE
    }

    #[inline(always)]
    pub fn alloc_unit(&self) -> TypeId {
        TypeId::UNIT
    }

    #[inline(always)]
    pub fn alloc_never(&self) -> TypeId {
        TypeId::NEVER
    }

    #[inline(always)]
    pub fn alloc_int(&mut self, width: u16) -> TypeId {
        self.get_or_insert(TypeKey::Int(width), TyKind::Int, width as u32)
    }

    #[inline(always)]
    pub fn alloc_uint(&mut self, width: u16) -> TypeId {
        self.get_or_insert(TypeKey::UInt(width), TyKind::UInt, width as u32)
    }

    #[inline(always)]
    pub fn alloc_float(&mut self, width: u16) -> TypeId {
        self.get_or_insert(TypeKey::Float(width), TyKind::Float, width as u32)
    }

    #[inline(always)]
    pub fn alloc_ptr(&mut self, inner: TypeId) -> TypeId {
        self.get_or_insert(TypeKey::Ptr(inner), TyKind::Ptr, inner.0)
    }

    #[inline(always)]
    pub fn sort_struct_fields(fields: &[(NameId, TypeId)]) -> Vec<(NameId, TypeId)> {
        let mut fields = fields.to_vec();
        fields.sort_by_key(|&(name_id, _)| name_id);
        fields
    }

    #[inline(always)]
    pub fn alloc_struct(&mut self, fields: &[(NameId, TypeId)]) -> TypeId {
        let sorted_fields = Self::sort_struct_fields(fields);
        let key = TypeKey::Struct(sorted_fields.clone().into()); // PERF

        if let Some(&id) = self.interned.get(&key) {
            return id;
        }

        let extra_len = self.store.extra.len() as u32;
        let fields_len = sorted_fields.len();

        self.store.extra.push(fields_len as u32);

        for (name, ty) in sorted_fields {
            self.store.extra.push(name.0);
            self.store.extra.push(ty.0);
        }

        let id = TypeId(self.store.push(TyKind::Struct, extra_len) as u32);

        self.interned.insert(key, id);
        id
    }

    #[inline(always)]
    pub fn get_struct_fields(&mut self, id: TypeId) -> Vec<(NameId, TypeId)> {
        let extra_idx = self.payload(id) as usize;
        let len = self.store.extra[extra_idx] as usize;

        let mut res = vec![];

        let offset = extra_idx + 1;

        for i in 0..len {
            res.push((
                NameId(self.store.extra[offset + (i * 2)]),
                TypeId(self.store.extra[offset + ((i * 2) + 1)]),
            ));
        }

        res
    }

    #[inline(always)]
    pub fn alloc_func(&mut self, params: &[TypeId], ret: TypeId, is_extern: bool) -> TypeId {
        let key = TypeKey::Func(params.into(), ret, is_extern);

        if let Some(&id) = self.interned.get(&key) {
            return id;
        }

        let extra_len = self.store.extra.len() as u32;

        // ------------
        let params_len = params.len() as u16;
        let is_extern = is_extern as u16;

        let packed = ((params_len as u32) << 16) | is_extern as u32;

        self.store.extra.push(packed);
        self.store.extra.push(ret.0);
        for p in params {
            self.store.extra.push(p.0);
        }
        // ---> [Header: u32] [return: TypeId] [param_n: TypeId] ...

        let id = TypeId(self.store.push(TyKind::Func, extra_len) as u32);

        self.interned.insert(key, id);

        id
    }

    #[inline(always)]
    pub fn func_return(&self, func: TypeId) -> TypeId {
        TypeId(self.store.extra[self.payload(func) as usize + 1])
    }

    #[inline(always)]
    pub fn func_params_len(&self, func: TypeId) -> u16 {
        let header_idx = self.payload(func) as usize;
        let header_data = self.store.extra[header_idx];

        (header_data >> 16) as u16
    }

    #[inline(always)]
    pub fn func_is_extern(&self, func: TypeId) -> bool {
        let header_idx = self.payload(func) as usize;
        let header_data = self.store.extra[header_idx];

        (header_data & 0xFFFF) != 0
    }

    #[inline(always)]
    pub fn func_params(&self, func: TypeId) -> Vec<TypeId> /* PREF: remove allocation and return an slice */
    {
        let header_idx = self.payload(func) as usize;
        let count = self.func_params_len(func) as usize;

        let start = header_idx + 2;
        let end = start + count;

        self.store.extra[start..end]
            .iter()
            .map(|v| TypeId(*v))
            .collect()
    }
}
