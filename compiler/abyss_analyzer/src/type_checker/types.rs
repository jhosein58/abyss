#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    I32,
    F32,
    Bool,
    Str,
    Cstr,
    Char,
    Unit,
    Error,
}
