use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

use crate::pass_synth::synth_node;

pub fn check(db: &mut Nexus, range: HirRange) {
    let start = range.start.0 as usize;
    let end = range.end.0 as usize;

    for offset in 0..=(end - start) {
        let id = HirId((start + offset) as u32);

        synth_node(db, id);
    }
}
