use anyhow::Result;

use crate::{deps::DepsGraph, locator::PackageLocations};

mod deps;
mod locator;
mod package;

fn main() -> Result<()> {
    let locs = PackageLocations::for_flake_spec("..#core")?;
    let graph = DepsGraph::from_locs(locs)?;

    println!("{}", graph.to_graphviz());

    Ok(())
}
