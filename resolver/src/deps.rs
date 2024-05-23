use petgraph::{Directed, Graph};

use crate::package::Package;

/// A built dependency graph, with no dangling references.
#[derive(Debug)]
pub struct DepsGraph {
    pub(crate) pkgs: Vec<Package>,
    pub(crate) g: Graph<(), (), Directed, usize>,
}

impl DepsGraph {}
