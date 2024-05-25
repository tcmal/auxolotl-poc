//! Code for extracting the positions of package lambdas.
//! This uses the `*.lambdas` outputs of the flake, and parses the output of `nix eval` to get the source locations.
//! Very hacky

use std::process::Command;

use anyhow::{anyhow, bail, Result};
use regex::Regex;
use rnix::ast::{self, AttrSet, Expr, HasEntry};

use crate::package::{Package, SourcePos};

/// A list of packages, with location information
#[derive(Debug, Clone)]
pub struct PackageLocations(Vec<Package>);

impl IntoIterator for PackageLocations {
    type Item = Package;

    type IntoIter = <Vec<Package> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PackageLocations {
    /// Extract package locations from a given flake specification
    /// The flake must have an output `.lambdas`.
    pub fn for_flake_spec(spec: &str, flake_slug: &str) -> Result<Self> {
        let out = Command::new("nix")
            .args(["eval", &format!("{spec}.lambdas")])
            .output()?;

        if !out.status.success() {
            bail!("error in nix eval");
        }
        let out = String::from_utf8(out.stdout)?;

        Self::from_eval_output(&out, flake_slug)
    }

    /// Extract locations of lambdas from the output of `nix eval`.
    pub fn from_eval_output(out: &str, flake_slug: &str) -> Result<Self> {
        // Replace <<lambda>> bits with strings
        let re = Regex::new(r"«lambda [^@]*@ ([^»]*)»").unwrap();
        let out = re.replace_all(out, r#""$1""#);

        let parse = rnix::Root::parse(&out);

        let Some(Expr::AttrSet(attrs)) = parse.tree().expr() else {
            bail!("result of evaluating lambdas was not an attribute set");
        };

        let mut this = Self(Default::default());
        this.walk_attrset("", &attrs, flake_slug)?;

        Ok(this)
    }

    /// Walk the parsed attribute set, adding encountered lambdas. `prefix` is used to deal with recursive attribute sets.
    fn walk_attrset(&mut self, prefix: &str, set: &AttrSet, flake_slug: &str) -> Result<()> {
        // note that we only have to parse the output of `nix eval`, so we can limit things a bit
        for entry in set.entries() {
            if let ast::Entry::AttrpathValue(attrpath_value) = entry {
                let attrpath = attrpath_value.attrpath().expect("value without attrpath");
                let ident = attrpath
                    .attrs()
                    .last()
                    .and_then(|attr| match attr {
                        ast::Attr::Ident(ident) => Some(ident),
                        _ => None,
                    })
                    .ok_or_else(|| anyhow!("todo: value with complex ident"))?;

                let name = ident.ident_token().unwrap().text().to_string();

                // TODO: deal with nested attribute sets
                let Some(Expr::Str(s)) = attrpath_value.value() else {
                    bail!("invalid value for attrpath value");
                };

                let pos = Self::get_lambda_position(s)
                    .ok_or_else(|| anyhow!("failed to extract lambda position"))?;

                self.0.push(Package {
                    name: format!("{prefix}{name}"),
                    pos,
                    flake_slug: flake_slug.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Extract the position of the lambda from a syntax tree entry.
    /// Since we're parsing the output of `nix eval`, this involves messing around with error nodes and stuff.
    /// This is really scuffed: there's probably a better way to do it.
    fn get_lambda_position(l: ast::Str) -> Option<SourcePos> {
        let parts = l.normalized_parts();
        let ast::InterpolPart::Literal(text) = parts.get(0)? else {
            return None;
        };
        let (file, text) = text.split_once(':')?;
        let (row, col) = text.split_once(':')?;

        return Some(SourcePos {
            file: file.to_string(),
            row: row.parse().ok()?,
            col: col.parse().ok()?,
        });
    }
}
