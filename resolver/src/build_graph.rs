//! Builds a dependency graph from static analysis of Nix code.

use std::{fs::File, io::Read};

use anyhow::{anyhow, bail, Context, Result};
use petgraph::Graph;
use rnix::{
    ast::{self, Expr, HasEntry},
    Root,
};
use rowan::{ast::AstNode, TextSize};

use crate::{deps::DepsGraph, locator::PackageLocations, package::Package};

/// A partially built dependency graph, which may contain dangling references.
#[derive(Debug, Clone, Default)]
pub struct PartialDepsGraph(Vec<(Package, Vec<String>)>);

impl PartialDepsGraph {
    /// Build a dependency graph from the given package locations.
    /// This expects each package declaration to point to a `callPackages` style lambda, which
    /// destructures its required dependencies by name.
    /// TODO: Later, this will attempt to read nested attributes by checking for `let inherit` forms right after.
    pub fn add_from_locs(&mut self, ps: PackageLocations) -> Result<()> {
        for p in ps {
            let deps = Self::get_dep_names(&p)
                .with_context(|| format!("error processing package {}", p.name))?;
            self.0.push((p, deps));
        }

        Ok(())
    }

    /// Get the names of all dependencies of the given package, by statically analysing nix code.
    fn get_dep_names(p: &Package) -> Result<Vec<String>> {
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

        Ok(deps)
    }

    /// Finalize the dependency graph, checking that there are no dangling references.
    pub fn finalize(mut self) -> Result<DepsGraph> {
        // Sort the list of packages, so that we get a consistent ordering between runs
        self.0.sort_by(|(p1, _), (p2, _)| p1.name.cmp(&p2.name));

        let g = self
            .0
            .iter()
            .enumerate()
            .flat_map(|(i, (p, deps))| {
                let this = &self;
                deps.iter().map(move |dep| -> Result<(usize, usize)> {
                    Ok((
                        i,
                        this.0
                            .iter()
                            .position(|(p, _)| p.name == *dep)
                            .ok_or_else(|| anyhow!("{} has missing dependency {}", p.name, dep))?,
                    ))
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let g = Graph::from_edges(g);

        Ok(DepsGraph {
            pkgs: self.0.into_iter().map(|(p, _)| p).collect(),
            g,
        })
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
