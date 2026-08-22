use abyss_nexus::{arena::ArenaId, nexus::HirId};

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_if(&mut self) -> HirId {
        self.bump();

        HirId::none()
    }
}
