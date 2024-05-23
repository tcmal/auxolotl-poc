//! Builds a dependency graph from static analysis of Nix code.

use std::{collections::HashMap, fs::File, io::Read};

use anyhow::{anyhow, bail, Result};
use petgraph::{
    dot::{Config, Dot},
    Graph,
};
use rnix::{
    ast::{self, Expr, HasEntry},
    Root,
};
use rowan::{ast::AstNode, TextSize};

use crate::{locator::PackageLocations, package::Package};

/// A dependency graph, containing one or more flakes
#[derive(Debug, Clone)]
pub struct DepsGraph(Graph<Package, usize>);

impl DepsGraph {
    /// Build a dependency graph from the given package locations.
    /// This expects each package declaration to point to a `callPackages` style lambda, which
    /// destructures its required dependencies by name.
    /// TODO: Later, this will attempt to read nested attributes by checking for `let inherit` forms right after.
    pub fn from_locs(ps: PackageLocations) -> Result<Self> {
        // We can't immediately build the graph because of forward referencing, so first build a map of name to dependencies
        let map = ps
            .into_iter()
            .map(|p| Self::get_dep_names(p))
            .collect::<Result<HashMap<_, _>>>()?;

        let mut g = Graph::new();

        // Insert each one into the graph
        let ni_map = map
            .iter()
            .map(|(p, _)| (&p.name, g.add_node(p.clone())))
            .collect::<HashMap<_, _>>();

        // Then actually create the nodes
        for (p, deps) in map.iter() {
            let from = ni_map.get(&p.name).unwrap();
            for dep in deps {
                let to = ni_map
                    .get(&dep)
                    .ok_or_else(|| anyhow!("{} has missing dependency {}", p.name, dep))?;
                g.add_edge(*from, *to, 0);
            }
        }

        Ok(Self(g))
    }

    /// Get the names of all dependencies of the given package, by statically analysing nix code.
    fn get_dep_names(p: Package) -> Result<(Package, Vec<String>)> {
        // load file
        let content = {
            let mut f = File::open(&p.pos.file)?;
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            content
        };
        let ast = Root::parse(&content).ok()?;

        // turn row and col into byte offset
        let offset = p
            .pos
            .to_offset(&content)
            .ok_or_else(|| anyhow!("source position invalid"))?;

        // skip to lambda
        let Some(Expr::Lambda(l)) = skip_to_offset(
            (offset as u32).into(),
            ast.expr().ok_or_else(|| anyhow!("file is empty"))?,
        ) else {
            bail!("lambda location doesn't point to a lambda");
        };

        let mut deps = Vec::new();
        // parse destructured arguments
        match l.param().unwrap() {
            ast::Param::Pattern(p) => {
                for e in p.pat_entries() {
                    let dep_name = e.ident().unwrap().ident_token().unwrap().text().to_string();
                    deps.push(dep_name);
                }
            }
            ast::Param::IdentParam(_) => {
                // no dependencies!
            }
        };

        // TODO: check for let inherit ()...

        Ok((p, deps))
    }

    /// Render the dependency graph as a graphviz string
    pub fn to_graphviz(&self) -> String {
        format!("{}", Dot::with_config(&self.0, &[Config::EdgeNoLabel]))
    }
}

/// Skip to the given offset within an expression tree
fn skip_to_offset(offset: TextSize, expr: Expr) -> Option<Expr> {
    match expr {
        Expr::Lambda(l) => {
            if l.param()?.syntax().text_range().contains(offset) {
                Some(Expr::Lambda(l))
            } else {
                skip_to_offset(offset, l.body()?)
            }
        }
        Expr::Apply(a) => {
            if a.lambda()?.syntax().text_range().contains(offset) {
                skip_to_offset(offset, a.lambda()?)
            } else {
                skip_to_offset(offset, a.argument()?)
            }
        }
        Expr::LetIn(li) => {
            if li.let_token()?.text_range().contains(offset) {
                Some(Expr::LetIn(li))
            } else if li.body()?.syntax().text_range().contains(offset) {
                skip_to_offset(offset, li.body()?)
            } else {
                todo!("look at entries of let in")
            }
        }
        Expr::AttrSet(a) => {
            for ent in a.entries() {
                match ent {
                    ast::Entry::Inherit(i) => {
                        if i.syntax().text_range().contains(offset) {
                            todo!("offset in inherit clause")
                        }
                    }
                    ast::Entry::AttrpathValue(av) => {
                        if av.syntax().text_range().contains(offset) {
                            return skip_to_offset(offset, av.value()?);
                        }
                    }
                }
            }
            None
        }
        // TODO: these arent yet needed, but might be
        Expr::Assert(_) => todo!(),
        Expr::Error(_) => todo!(),
        Expr::IfElse(_) => todo!(),
        Expr::Select(_) => todo!(),
        Expr::Str(_) => todo!(),
        Expr::Path(_) => todo!(),
        Expr::Literal(_) => todo!(),
        Expr::LegacyLet(_) => todo!(),
        Expr::List(_) => todo!(),
        Expr::BinOp(_) => todo!(),
        Expr::Paren(_) => todo!(),
        Expr::Root(_) => todo!(),
        Expr::UnaryOp(_) => todo!(),
        Expr::Ident(_) => todo!(),
        Expr::With(_) => todo!(),
        Expr::HasAttr(_) => todo!(),
    }
}
