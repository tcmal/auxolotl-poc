use std::{
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, Result};
use deps::DepsGraph;

use crate::{build_graph::PartialDepsGraph, locator::PackageLocations};

mod build_graph;
mod codegen;
mod deps;
mod locator;
mod package;

const GRAPH_OUT: &'static str = "./registry/pkgs.svg";
const NIX_OUT: &'static str = "./registry/pkgs.nix";

fn main() -> Result<()> {
    let locs = PackageLocations::for_flake_spec("./registry")?;

    println!("Found {} packages", locs.len());

    let mut graph = PartialDepsGraph::default();
    graph.add_from_locs(locs)?;
    let graph = graph.finalize()?;
    println!("Dependency graph built successfully.");

    if let Err(e) = render_graph(GRAPH_OUT, &graph) {
        println!("Error rendering graph: {:?}", e);
    } else {
        println!("Rendered graph to {}", GRAPH_OUT);
    }

    if let Err(e) = render_nix(NIX_OUT, &graph) {
        println!("Error rendering nix code: {:?}", e);
    } else {
        println!("Rendered nix code to {}", NIX_OUT);
    }

    Ok(())
}

fn render_nix(out: &str, graph: &DepsGraph) -> Result<()> {
    let levels = graph.to_levels();
    let code = graph.levels_to_nix(&levels);

    let mut f = File::create(out)?;
    f.write_all(code.as_bytes())?;

    if !Command::new("nixfmt")
        .args([out])
        .spawn()?
        .wait()?
        .success()
    {
        bail!("error code from nixfmt");
    }

    Ok(())
}

fn render_graph(out: &str, g: &DepsGraph) -> Result<()> {
    let mut child = Command::new("dot")
        .args([&format!("-o{out}"), "-Tsvg"])
        .stdin(Stdio::piped())
        .spawn()?;

    let gviz = g.to_graphviz();
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow!("Failed to open stdin"))?;
    std::thread::spawn(move || {
        stdin
            .write_all(gviz.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait()?;
    if !output.success() {
        bail!("error running graphviz");
    }

    Ok(())
}
