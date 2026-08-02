use crate::kind::TokenKind;

#[derive(Default, Debug)]
pub struct TokenStream<'a> {
    pub kinds: Vec<TokenKind>,
    pub texts: Vec<&'a str>,
    pub starts: Vec<usize>,
    pub lens: Vec<usize>,
    pub preceded_by_newlines: Vec<bool>,
}

impl<'a> TokenStream<'a> {
    pub fn push(
        &mut self,
        kind: TokenKind,
        text: &'a str,
        start: usize,
        len: usize,
        preceded_by_newline: bool,
    ) {
        self.kinds.push(kind);
        self.texts.push(text);
        self.starts.push(start);
        self.lens.push(len);
        self.preceded_by_newlines.push(preceded_by_newline);
    }
}
