#[derive(Debug, Clone)]
pub struct IdGenerator {
    current_id: u32,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { current_id: 1 }
    }

    pub fn next(&mut self) -> u32 {
        let id = self.current_id;

        self.current_id = self.current_id.checked_add(1).expect(
            "Node ID overflow! The compiler exceeded the maximum number of AST/TAST nodes (4.29 billion)."
        );

        id
    }

    pub fn peek(&self) -> u32 {
        self.current_id
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}
