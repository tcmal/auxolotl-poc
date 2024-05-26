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
    /// The flake must have an output `.lambdas`, which is an attribute set mapping `<flake>.<package path>` to the callPackage-style lambda corresponding to that package.
    pub fn for_flake_spec(spec: &str) -> Result<Self> {
        let out = Command::new("nix")
            .args(["eval", &format!("{spec}#lambdas")])
            .output()?;

        let outp = String::from_utf8(out.stdout)?;
        if !out.status.success() {
            bail!(
                "error in nix eval: {} {}",
                outp,
                if let Ok(s) = String::from_utf8(out.stderr) {
                    s
                } else {
                    "invalid utf8 in stderr".to_string()
                }
            );
        }

        Self::from_eval_output(&outp)
    }

    /// Extract locations of lambdas from the output of `nix eval registry#lambdas`.
    pub fn from_eval_output(out: &str) -> Result<Self> {
        // Replace <<lambda>> bits with strings
        let re = Regex::new(r"«lambda [^@]*@ ([^»]*)»").unwrap();
        let out = re.replace_all(out, r#""$1""#);

        let parse = rnix::Root::parse(&out);

        let Some(Expr::AttrSet(attrs)) = parse.tree().expr() else {
            bail!("result of evaluating lambdas was not an attribute set");
        };

        let mut this = Self(Default::default());
        this.walk_attrset("", &attrs)?;

        Ok(this)
    }

    /// Walk the parsed attribute set, adding encountered lambdas. `prefix` is used to deal with recursive attribute sets.
    fn walk_attrset(&mut self, prefix: &str, set: &AttrSet) -> Result<()> {
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
                match attrpath_value.value() {
                    // Position of lambda (thanks to earlier regex replacement)
                    Some(Expr::Str(s)) => {
                        let pos = Self::get_lambda_position(s)
                            .ok_or_else(|| anyhow!("failed to extract lambda position"))?;

                        let full_path = format!("{prefix}{name}");

                        let (flake_slug, name) = full_path
                            .split_once(".")
                            .expect("every package must be nested at least once");
                        self.0.push(Package {
                            name: name.to_string(),
                            flake_slug: flake_slug.to_string(),
                            pos,
                        });
                    }

                    // Nested attribute set
                    Some(Expr::AttrSet(a)) => {
                        self.walk_attrset(&format!("{prefix}{ident}."), &a)?
                    }

                    x => bail!("unrecognised value type for attribute: {:?}", x),
                }
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
