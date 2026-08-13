use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

use crate::{pass_check::check_node, pass_synth::synth_node};

pub fn type_check(db: &mut Nexus, range: HirRange) {
    let start = range.start.0;
    let end = range.end.0;

    // check
    for offset in (0..=(end - start)).rev() {
        check_node(db, cal_id(offset, start));
    }

    // synth
    for offset in 0..=(end - start) {
        synth_node(db, cal_id(offset, start));
    }
}

#[inline(always)]
fn cal_id(offset: u32, start: u32) -> HirId {
    HirId(start + offset)
}
