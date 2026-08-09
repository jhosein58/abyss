use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

pub fn check(db: &mut Nexus, range: HirRange) {
    let start_idx = range.start.0 as usize;
    let end_idx = range.end.0 as usize;

    let kinds_slice = &db.hir.table.kinds[start_idx..=end_idx];

    for (offset, kind) in kinds_slice.iter().enumerate() {
        let id = HirId((start_idx + offset) as u32);

        println!("ID: {:?}, Kind: {:?}", id, kind);
    }
}
