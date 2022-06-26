use std::{collections::HashMap, rc::Rc};

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
    // adjacency list which represents variant dependencies
    dep: HashMap<Adt, Vec<Adt>>,
}

impl SourceInfo {
    pub fn new() -> Self {
        SourceInfo {
            dep: HashMap::new(),
        }
    }

    pub fn register_adt(&mut self, adt: Adt) {
        self.dep.insert(adt, vec![]);
    }

    pub fn add_dependency(&mut self, parent: &Adt, child: Adt) {
        let orig = self.dep.get_mut(parent).unwrap();
        orig.push(child);
    }
}
