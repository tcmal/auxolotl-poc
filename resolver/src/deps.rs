use petgraph::{
    dot::{Config, Dot},
    visit::{EdgeRef, Topo},
    Directed, Direction, Graph,
};

use crate::package::Package;

pub type PkgRef = usize;

/// A built dependency graph, with no dangling references.
#[derive(Debug)]
pub struct DepsGraph {
    /// A flat list of packages, sorted alphabetically.
    pub(crate) pkgs: Vec<Package>,
    // TODO: the usizes arent really necessary, but `Dot` doesn't implement Display if we use ()
    /// The dependency graph. Each node corresponds to the package with the same index, and an edge from `a` to `b` implies that `b` depends on `a`
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
                &|_, (i, _)| {
                    let p = &self.pkgs[i.index()];
                    format!("label=\"{}.{}\"", &p.flake_slug, &p.name)
                },
            )
        )
    }

    /// Get a reference to the package with the given reference.
    /// Panics if the reference is invalid.
    pub fn get(&self, pkg: PkgRef) -> &Package {
        &self.pkgs[pkg]
    }

    /// Convert the dependency tree to a list of lists, where each item's dependencies are all in an earlier list.
    // TODO: Currently this is somewhat pessimistic, there's probably ways to make things a bit flatter
    // (and also improve the algorithm's performance)
    pub fn to_levels(&self) -> Vec<Vec<PkgRef>> {
        let mut visitor = Topo::new(&self.g);
        let mut levels = vec![vec![]];
        while let Some(node) = visitor.next(&self.g) {
            // if any dependencies are in the current level, start a new level
            for edge in self.g.edges_directed(node, Direction::Incoming) {
                if levels[levels.len() - 1].contains(&edge.source().index()) {
                    levels.push(vec![]);
                    break;
                }
            }
            let last = levels.len() - 1;
            levels[last].push(node.index());
        }

        if levels.iter().map(|l| l.len()).sum::<usize>() != self.pkgs.len() {
            // TODO: this might be valid in some cases? not certain though
            unimplemented!("topological walking missed some packages, indicating a cyclic graph");
        }

        levels
    }
}
