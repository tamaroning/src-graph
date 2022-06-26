use rustc_ap_graphviz as dot;
use std::{fs::File, io::Write, path::Path};

type Nd = isize;
type Ed = (isize, isize);
struct Edges(Vec<Ed>);

fn render_to<W: Write>(output: &mut W) {
    let edges = Edges(vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 4)]);
    dot::render(&edges, output).unwrap()
}

impl<'a> dot::Labeller<'a> for Edges {
    type Node = Nd;
    type Edge = Ed;
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("crate").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", *n)).unwrap()
    }
}

impl<'a> dot::GraphWalk<'a> for Edges {
    type Node = Nd;
    type Edge = Ed;
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        // (assumes that |N| \approxeq |E|)
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());
        for &(s, t) in v {
            nodes.push(s);
            nodes.push(t);
        }
        nodes.sort();
        nodes.dedup();
        nodes.into()
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let &Edges(ref edges) = self;
        (&edges[..]).into()
    }

    fn source(&self, e: &Ed) -> Nd {
        let &(s, _) = e;
        s
    }

    fn target(&self, e: &Ed) -> Nd {
        let &(_, t) = e;
        t
    }
}

pub fn output_dot(path: &Path) {
    let mut file = File::create(path).unwrap();
    render_to(&mut file)
}
