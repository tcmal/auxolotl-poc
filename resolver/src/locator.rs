//! Code for extracting the positions of package lambdas.
//! This uses the `*.lambdas` outputs of the flake.

use std::{collections::HashMap, process::Command};

use anyhow::{anyhow, bail, Result};
use regex::Regex;
use rnix::ast::{self, AttrSet, Expr, HasEntry};

#[derive(Debug, Clone)]
pub struct LambdaLocs(HashMap<String, SourcePos>);

/// Identififes a position in nix code
#[derive(Debug, Clone)]
pub struct SourcePos {
    pub file: String,
    pub row: usize,
    pub col: usize,
}

impl LambdaLocs {
    pub fn for_flake_spec(spec: &str) -> Result<Self> {
        let out = Command::new("nix")
            .args(["eval", &format!("{spec}.lambdas")])
            .output()?;

        if !out.status.success() {
            bail!("error in nix eval");
        }
        let out = String::from_utf8(out.stdout)?;

        Self::from_eval_output(&out)
    }

    /// Extract locations of lambdas from the output of `nix eval`.
    pub fn from_eval_output(out: &str) -> Result<Self> {
        // Replace <<lambda>> bits with strings
        let re = Regex::new(r"«lambda [^@]*@ ([^»]*)»").unwrap();
        let out = re.replace_all(out, r#""$1""#);

        let parse = rnix::Root::parse(&out);

        let Some(Expr::AttrSet(attrs)) = parse.tree().expr() else {
            bail!("result of evaluating lambdas was not an attribute set");
        };

        let mut this = Self(HashMap::new());
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

                let Some(Expr::Str(s)) = attrpath_value.value() else {
                    bail!("invalid value for attrpath value");
                };

                let pos = Self::get_lambda_position(s)
                    .ok_or_else(|| anyhow!("failed to extract lambda position"))?;

                self.0.insert(format!("{prefix}{name}"), pos);
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
