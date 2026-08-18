use abyss_nexus::{nexus::Nexus, ranges::HirRange};

use crate::{pass_check::check_all, pass_synth::synth_all};

pub fn type_check(db: &mut Nexus, range: HirRange) {
    check_all(db, range);
    synth_all(db, range);
}
