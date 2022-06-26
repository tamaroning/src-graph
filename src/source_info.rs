use std::collections::{HashMap, HashSet};

type Deps<T> = HashMap<T, HashSet<T>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Adt {
    pub name: String,
}

impl Adt {
    pub fn new(name: String) -> Self {
        Adt { name }
    }
}

#[derive(Debug)]
pub struct SourceInfo {
    // adjacency list which represents ADT dependencies
    deps: Deps<Adt>,
}

impl SourceInfo {
    pub fn new() -> Self {
        SourceInfo { deps: Deps::new() }
    }

    pub fn register_adt(&mut self, adt: Adt) {
        self.deps.insert(adt, HashSet::new());
    }

    pub fn add_dependency(&mut self, parent: &Adt, child: Adt) {
        let orig = self.deps.get_mut(parent).unwrap();
        orig.insert(child);
    }

    pub fn deps(&self) -> Deps<Adt> {
        self.deps.clone()
    }
}
