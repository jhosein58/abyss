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

    pub fn append(&mut self, mut other: TokenStream<'a>) {
        let additional = other.kinds.len();
        self.kinds.reserve(additional);
        self.texts.reserve(additional);
        self.starts.reserve(additional);
        self.lens.reserve(additional);
        self.preceded_by_newlines.reserve(additional);

        self.kinds.append(&mut other.kinds);
        self.texts.append(&mut other.texts);
        self.starts.append(&mut other.starts);
        self.lens.append(&mut other.lens);
        self.preceded_by_newlines
            .append(&mut other.preceded_by_newlines);
    }
}
