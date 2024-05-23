use petgraph::{
    dot::{Config, Dot},
    Directed, Graph,
};

use crate::package::Package;

/// A built dependency graph, with no dangling references.
#[derive(Debug)]
pub struct DepsGraph {
    pub(crate) pkgs: Vec<Package>,
    // TODO: the usizes arent really necessary, but `Dot` doesn't implement Display if we use ()
    pub(crate) g: Graph<usize, usize, Directed, usize>,
}

impl DepsGraph {
    /// Return a graphviz representation of the graph
    pub fn to_graphviz(&self) -> String {
        format!(
            "{}",
            Dot::with_attr_getters(
                &self.g,
                &[Config::EdgeNoLabel, Config::NodeNoLabel],
                &|_, _| String::new(),
                &|_, (i, _)| { format!("label=\"{}\"", self.pkgs[i.index()].name.to_string()) },
            )
        )
    }
}
