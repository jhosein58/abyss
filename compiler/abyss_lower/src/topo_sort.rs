use std::collections::HashSet;

use abyss_nexus::nexus::{Nexus, TypeId};
use abyss_types::TyKind;

pub fn get_deps(db: &Nexus, tyid: TypeId) -> Vec<TypeId> {
    let kind = db.types.kind(tyid);

    match kind {
        TyKind::Struct => {
            let mut deps = vec![];
            let fields = db.types.get_struct_fields(tyid);

            for (_, t) in fields {
                if db.types.kind(t) == TyKind::Struct {
                    deps.push(t);
                }
            }

            deps
        }
        _ => {
            vec![]
        }
    }
}

pub fn topo_sort(db: &Nexus, list: &HashSet<TypeId>) -> Vec<TypeId> {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum State {
        Unsorted,
        Sorting,
        Sorted,
    }

    let mut sorted = vec![];
    let mut states = vec![State::Unsorted; db.types.len()];

    fn dfs(db: &Nexus, sorted: &mut Vec<TypeId>, states: &mut Vec<State>, id: TypeId) {
        match states[id.0 as usize] {
            State::Sorted => return,
            State::Sorting => panic!(),
            State::Unsorted => {
                states[id.0 as usize] = State::Sorting;
                let deps = get_deps(db, id);

                for d in deps {
                    dfs(db, sorted, states, d);
                }

                sorted.push(id);

                states[id.0 as usize] = State::Sorted;
            }
        }
    }

    for t in list {
        dfs(db, &mut sorted, &mut states, *t);
    }

    sorted
}
