use std::collections::HashMap;

use abyss_types::{TyKind, TyStore};

use crate::{arena::ArenaId, nexus::TypeId};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)] // FIXME: impl trait Copy
pub enum TypeKey {
    Unknown,

    Int(u16), // bit width
    UInt(u16),
    Float(u16),
    Bool,
    Ptr(TypeId),
    Type,
    Unit,
    Func(Box<[TypeId]>, TypeId), // PERF: Box ro hazf kon

    Error,
}

#[derive(Default)]
pub struct TypeStorage {
    store: TyStore,
    interned: HashMap<TypeKey, TypeId>, // PERF: hash-map inja mitone sari tar beshe ya kolan hazf beshe
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
        self.store.kinds[idx.0 as usize]
    }

    #[inline(always)]
    pub fn payload(&self, idx: TypeId) -> u32 {
        self.store.payload[idx.0 as usize]
    }

    pub fn name(&self, idx: TypeId) -> String {
        if idx.is_none() {
            return "None".to_string();
        }

        match self.kind(idx) {
            TyKind::Unknown => "Unknown".to_string(),

            TyKind::Int => format!("i{}", self.payload(idx)),
            TyKind::UInt => format!("u{}", self.payload(idx)),
            TyKind::Float => format!("f{}", self.payload(idx)),
            TyKind::Bool => format!("bool"),
            TyKind::Ptr => format!("&{}", self.name(TypeId(self.payload(idx)))),
            TyKind::Type => format!("Type({})", self.name(TypeId(self.payload(idx)))),
            TyKind::Unit => format!("unit"),
            TyKind::Func => format!("fn"),
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
    pub fn alloc_unknown(&mut self) -> TypeId {
        self.get_or_insert(TypeKey::Unknown, TyKind::Unknown, 0)
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
    pub fn alloc_bool(&mut self) -> TypeId {
        self.get_or_insert(TypeKey::Bool, TyKind::Bool, 0)
    }

    #[inline(always)]
    pub fn alloc_ptr(&mut self, inner: TypeId) -> TypeId {
        self.get_or_insert(TypeKey::Ptr(inner), TyKind::Ptr, inner.0)
    }

    #[inline(always)]
    pub fn alloc_error(&mut self) -> TypeId {
        self.get_or_insert(TypeKey::Error, TyKind::Error, 0)
    }

    // == Functions =>

    #[inline(always)]
    pub fn alloc_func(&mut self, params: &[TypeId], ret: TypeId) -> TypeId {
        let key = TypeKey::Func(params.into(), ret);

        if let Some(&id) = self.interned.get(&key) {
            return id;
        }

        let extra_len = self.store.extra.len() as u32;

        // ------------
        self.store.extra.push(params.len() as u32); // IDEA: u16 kafie baraye tool arg. 2 byte dari ke mitoni tosh meta-data benevisi
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
    pub fn func_return(&mut self, func: TypeId) -> TypeId {
        TypeId(self.store.extra[self.payload(func) as usize + 1])
    }

    #[inline(always)]
    pub fn func_params(&mut self, func: TypeId) -> Vec<TypeId> /* PREF: remove allocation and return an slice */
    {
        let header_idx = self.payload(func) as usize;
        let header = self.store.extra[header_idx]; // FIXME: unpack header logic

        self.store.extra[(header_idx + 2)..header as usize]
            .into_iter()
            .map(|v| TypeId(*v))
            .collect()
    }
}
