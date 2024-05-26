use anyhow::{Context, Result};

use crate::{build_graph::PartialDepsGraph, locator::PackageLocations};

mod build_graph;
mod codegen;
mod deps;
mod locator;
mod package;

fn main() -> Result<()> {
    let mut graph = PartialDepsGraph::default();

    graph.add_from_locs(PackageLocations::for_flake_spec("./registry")?)?;

    let graph = graph.finalize()?;
    let levels = graph.to_levels();

    println!("{}", graph.levels_to_nix(&levels));

    Ok(())
}
