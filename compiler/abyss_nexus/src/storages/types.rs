use std::collections::HashMap;

use abyss_types::{TyKind, TyStore};

use crate::nexus::TypeId;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TypeKey {
    Unknown,

    Int(u16), // bit width
    UInt(u16),
    Float(u16),
    Bool,

    Ptr(TypeId),

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
        match self.kind(idx) {
            TyKind::Unknown => "Unknown".to_string(),

            TyKind::Int => format!("i{}", self.payload(idx)),
            TyKind::UInt => format!("u{}", self.payload(idx)),
            TyKind::Float => format!("f{}", self.payload(idx)),
            TyKind::Bool => format!("bool"),
            TyKind::Ptr => format!("p{}", self.name(TypeId(self.payload(idx)))),
            TyKind::Error => format!("Err!"),

            _ => "<NOT_IMPLEMENTED>".to_string(),
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
}
