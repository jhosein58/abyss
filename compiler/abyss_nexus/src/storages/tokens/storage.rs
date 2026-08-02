use abyss_token::stream::TokenStream;

#[derive(Default)]
pub struct TokenStorage {
    pub stream: TokenStream<'static>,
}

impl TokenStorage {}
