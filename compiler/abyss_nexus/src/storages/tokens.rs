use abyss_token::{kind::TokenKind, stream::TokenStream};

use crate::arena_id;

arena_id!(TokenId);

#[derive(Default)]
pub struct TokenStorage {
    pub stream: TokenStream<'static>,
}

impl TokenStorage {
    #[inline]
    pub fn count(&self) -> usize {
        self.stream.kinds.len()
    }

    #[inline]
    pub fn kind(&self, id: TokenId) -> TokenKind {
        self.stream.kinds[id.0 as usize]
    }

    #[inline]
    pub fn text(&self, id: TokenId) -> &str {
        self.stream.texts[id.0 as usize]
    }

    #[inline]
    pub fn start(&self, id: TokenId) -> usize {
        self.stream.starts[id.0 as usize]
    }

    #[inline]
    pub fn len(&self, id: TokenId) -> usize {
        self.stream.lens[id.0 as usize]
    }

    #[inline]
    pub fn preceded_by_newline(&self, id: TokenId) -> bool {
        self.stream.preceded_by_newlines[id.0 as usize]
    }

    pub fn append(&mut self, other: TokenStream<'static>) {
        self.stream.append(other);
    }
}
