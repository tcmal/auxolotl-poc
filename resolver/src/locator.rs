//! Code for extracting the positions of package lambdas.
//! This uses the `*.lambdas` outputs of the flake.

use std::{collections::HashMap, process::Command};

use anyhow::{anyhow, bail, Result};
use rnix::{
    ast::{self, AttrSet, Expr, HasEntry, Lambda},
    SyntaxKind, SyntaxNode,
};
use rowan::{ast::AstNode, WalkEvent};

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
        let parse = rnix::Root::parse(out);
        // parse will contain errors, because of the <<lambda>> bits.

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

                let Some(l) = attrpath_value.value() else {
                    bail!("invalid value for attrpath value");
                };

                let Some(pos) = Self::get_lambda_position(l.syntax()) else {
                    bail!("failed to extract position of lambda");
                };

                self.0.insert(format!("{prefix}{name}"), pos);
            }
        }

        Ok(())
    }

    /// Extract the position of the lambda from a syntax tree entry.
    /// Since we're parsing the output of `nix eval`, this involves messing around with error nodes and stuff.
    /// This is really scuffed: there's probably a better way to do it.
    fn get_lambda_position(l: &SyntaxNode) -> Option<SourcePos> {
        for c in l.preorder() {
            if let WalkEvent::Enter(c) = c {
                if c.kind() != SyntaxKind::NODE_ERROR {
                    continue;
                }
                let text = dbg!(c.text().to_string());
                let (file, text) = text.split_once(':')?;
                let (row, text) = text.split_once(':')?;
                let (col, _) = text.split_once('»')?;

                return Some(SourcePos {
                    file: file.to_string(),
                    row: row.parse().ok()?,
                    col: col.parse().ok()?,
                });
            }
        }

        None
    }
}
