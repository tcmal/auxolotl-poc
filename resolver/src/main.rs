use anyhow::{Context, Result};

use crate::{build_graph::PartialDepsGraph, locator::PackageLocations};

mod build_graph;
mod deps;
mod locator;
mod package;

fn main() -> Result<()> {
    let mut graph = PartialDepsGraph::default();

    for flake in ["..#core", "..#extra"] {
        graph
            .add_from_locs(
                PackageLocations::for_flake_spec(flake)
                    .with_context(|| format!("error getting locations for flake {flake}"))?,
            )
            .with_context(|| format!("error getting dependencies for flake {flake}"))?;
    }

    dbg!(graph.finalize());

    Ok(())
}
