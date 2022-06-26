use dot::{render, GraphWalk, Labeller, LabelText};
use rustc_ap_graphviz as dot;
use std::{borrow::Cow, fs::File, path::Path};

use crate::source_info::SourceInfo;

type Nd = String;
type Ed = (String, String);
struct Edges(Vec<Ed>);

impl<'a> Labeller<'a> for Edges {
    type Node = Nd;
    type Edge = Ed;
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("crate").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(n.clone()).unwrap()
    }

    fn node_shape(&'a self, _node: &Self::Node) -> Option<dot::LabelText<'a>> {
        Some(LabelText::label("box"))
    }
}

impl<'a> GraphWalk<'a> for Edges {
    type Node = Nd;
    type Edge = Ed;
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len() * 2);
        for (s, t) in v {
            nodes.push(s.clone());
            nodes.push(t.clone());
        }
        nodes.sort();
        nodes.dedup();
        Cow::from(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let &Edges(ref edges) = self;
        (&edges[..]).into()
    }

    fn source(&self, e: &Ed) -> Nd {
        let (s, _) = &e;
        s.clone()
    }

    fn target(&self, e: &Ed) -> Nd {
        let (_, t) = &e;
        t.clone()
    }
}

// TODO: remove clone and abstract types
pub fn output_dot(output: &Path, info: &SourceInfo) {
    let mut output_file = File::create(output).unwrap();
    let mut edges = vec![];
    for (parent, children) in info.deps().iter() {
        for child in children.iter() {
            edges.push((parent.clone(), child.clone()));
        }
    }
    let edges = Edges(edges);
    render(&edges, &mut output_file).unwrap()
}
