use std::{collections::{HashMap, HashSet}, rc::Rc};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Adt {
    name: String
}

impl Adt {
    pub fn new(name: String) -> Self {
        Adt {name}
    }
}

#[derive(Debug)]
pub struct SourceInfo {
    // adjacency list which represents ADT dependencies
    dep: HashMap<Adt, HashSet<Adt>>,
}

impl SourceInfo {
    pub fn new() -> Self {
        SourceInfo {
            dep: HashMap::new(),
        }
    }

    pub fn register_adt(&mut self, adt: Adt) {
        self.dep.insert(adt, HashSet::new());
    }

    pub fn add_dependency(&mut self, parent: &Adt, child: Adt) {
        let orig = self.dep.get_mut(parent).unwrap();
        orig.insert(child);
    }
}
