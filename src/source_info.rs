use std::collections::{HashMap, HashSet};

type Deps<T> = HashMap<T, HashSet<T>>;

#[derive(Debug)]
pub struct SourceInfo {
    // adjacency list which represents ADT dependencies
    deps: Deps<String>,
}

impl SourceInfo {
    pub fn new() -> Self {
        SourceInfo { deps: Deps::new() }
    }

    pub fn register_adt(&mut self, adt: String) {
        self.deps.insert(adt, HashSet::new());
    }

    pub fn add_dependency(&mut self, parent: &String, child: String) {
        let orig = self.deps.get_mut(parent).unwrap();
        orig.insert(child);
    }

    pub fn deps(&self) -> Deps<String> {
        self.deps.clone()
    }
}
