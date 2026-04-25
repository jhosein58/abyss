use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: u32,
    pub text: &'a str,
    pub start: usize,
    pub len: usize,
}

#[derive(Clone, Copy)]
enum ByteOp {
    Exact(u8),
    Digit,
    Alpha,
    AlphaNum,
    Space,
    Any,
    Until(u8),
}

#[derive(Clone, Copy)]
enum Quant {
    One,
    Star,
    Plus,
}

#[derive(Clone, Copy)]
struct Node {
    op: ByteOp,
    q: Quant,
}

#[derive(Clone)]
pub struct Rule {
    kind: u32,
    nodes: Box<[Node]>,
}

pub struct DynamicLexerRules {
    kind_to_id: HashMap<String, u32>,
    pub id_to_kind: Vec<String>,
    pub rules: Vec<Rule>,
    dispatch: [Vec<usize>; 256],
}

impl Default for DynamicLexerRules {
    fn default() -> Self {
        let mut s = Self {
            kind_to_id: HashMap::new(),
            id_to_kind: vec!["Error".into()],
            rules: Vec::new(),
            dispatch: std::array::from_fn(|_| Vec::new()),
        };
        s.kind_to_id.insert("Error".into(), 0);
        s
    }
}

impl DynamicLexerRules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&mut self, kind: &str) -> u32 {
        if let Some(&v) = self.kind_to_id.get(kind) {
            return v;
        }

        let id = self.id_to_kind.len() as u32;
        self.kind_to_id.insert(kind.to_string(), id);
        self.id_to_kind.push(kind.to_string());
        id
    }

    pub fn add_token(&mut self, kind: &str, pattern: &str) {
        let kind_id = self.id(kind);
        let nodes = self.compile(pattern);

        let idx = self.rules.len();
        self.rules.push(Rule {
            kind: kind_id,
            nodes: nodes.into_boxed_slice(),
        });

        if let Some(n) = self.rules[idx].nodes.first() {
            self.dispatch_rule(idx, *n);
        }
    }

    fn dispatch_rule(&mut self, idx: usize, node: Node) {
        if matches!(node.q, Quant::Star) {
            for i in 0..=255 {
                self.dispatch[i].push(idx);
            }
            return;
        }

        match node.op {
            ByteOp::Exact(b) => self.dispatch[b as usize].push(idx),
            ByteOp::Digit => {
                for b in b'0'..=b'9' {
                    self.dispatch[b as usize].push(idx);
                }
            }
            ByteOp::Alpha => {
                for b in b'a'..=b'z' {
                    self.dispatch[b as usize].push(idx);
                }
                for b in b'A'..=b'Z' {
                    self.dispatch[b as usize].push(idx);
                }
                self.dispatch[b'_' as usize].push(idx);
            }
            ByteOp::AlphaNum => {
                for b in b'a'..=b'z' {
                    self.dispatch[b as usize].push(idx);
                }
                for b in b'A'..=b'Z' {
                    self.dispatch[b as usize].push(idx);
                }
                for b in b'0'..=b'9' {
                    self.dispatch[b as usize].push(idx);
                }
                self.dispatch[b'_' as usize].push(idx);
            }
            ByteOp::Space => {
                self.dispatch[b' ' as usize].push(idx);
                self.dispatch[b'\n' as usize].push(idx);
                self.dispatch[b'\t' as usize].push(idx);
                self.dispatch[b'\r' as usize].push(idx);
            }
            ByteOp::Any | ByteOp::Until(_) => {
                for i in 0..=255 {
                    self.dispatch[i].push(idx);
                }
            }
        }
    }

    fn compile(&self, pat: &str) -> Vec<Node> {
        let mut out = Vec::new();
        let bytes = pat.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut op = ByteOp::Exact(bytes[i]);

            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                i += 1;
                op = match bytes[i] {
                    b'd' => ByteOp::Digit,
                    b'a' => ByteOp::Alpha,
                    b'w' => ByteOp::AlphaNum,
                    b's' => ByteOp::Space,
                    x => ByteOp::Exact(x),
                };
            } else if bytes[i] == b'.' {
                op = ByteOp::Any;
            } else if bytes[i] == b'~' && i + 1 < bytes.len() {
                i += 1;
                op = ByteOp::Until(bytes[i]);
            }

            let mut q = Quant::One;

            if i + 1 < bytes.len() {
                match bytes[i + 1] {
                    b'*' => {
                        q = Quant::Star;
                        i += 1;
                    }
                    b'+' => {
                        q = Quant::Plus;
                        i += 1;
                    }
                    _ => {}
                }
            }

            out.push(Node { op, q });
            i += 1;
        }

        out
    }

    #[inline(always)]
    fn test(op: ByteOp, b: u8) -> bool {
        match op {
            ByteOp::Exact(x) => b == x,
            ByteOp::Digit => b.is_ascii_digit(),
            ByteOp::Alpha => b.is_ascii_alphabetic() || b == b'_',
            ByteOp::AlphaNum => b.is_ascii_alphanumeric() || b == b'_',
            ByteOp::Space => b.is_ascii_whitespace(),
            ByteOp::Any => true,
            ByteOp::Until(x) => b != x,
        }
    }

    #[inline(always)]
    fn match_rule(nodes: &[Node], input: &[u8]) -> Option<usize> {
        let mut i = 0;

        for n in nodes {
            let mut count = 0;

            while i < input.len() && Self::test(n.op, input[i]) {
                i += 1;
                count += 1;

                if matches!(n.q, Quant::One) {
                    break;
                }
            }

            match n.q {
                Quant::One if count != 1 => return None,
                Quant::Plus if count == 0 => return None,
                _ => {}
            }
        }

        Some(i)
    }
}

pub struct Scanner<'a> {
    src: &'a str,
    off: usize,
    pub rules: DynamicLexerRules,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str, rules: DynamicLexerRules) -> Self {
        Self { src, off: 0, rules }
    }

    #[inline]
    pub fn get_offset(&self) -> usize {
        self.off
    }

    #[inline]
    pub fn set_offset(&mut self, offset: usize) {
        self.off = offset;
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        if self.off >= self.src.len() {
            return None;
        }

        let bytes = self.src.as_bytes();
        let start = self.off;
        let first = bytes[start];

        let mut best_len = 0;
        let mut best_kind = 0;

        for &r in &self.rules.dispatch[first as usize] {
            let rule = &self.rules.rules[r];

            if let Some(len) = DynamicLexerRules::match_rule(&rule.nodes, &bytes[start..]) {
                if len >= best_len {
                    best_len = len;
                    best_kind = rule.kind;
                }
            }
        }

        if best_len == 0 {
            let c = self.src[start..].chars().next().unwrap();
            best_len = c.len_utf8();
            best_kind = 0;
        }

        self.off += best_len;

        Some(Token {
            kind: best_kind,
            text: &self.src[start..start + best_len],
            start,
            len: best_len,
        })
    }

    pub fn add_token(&mut self, kind: &str, pattern: &str) {
        self.rules.add_token(kind, pattern);
    }
}
