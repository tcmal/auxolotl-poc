use petgraph::{visit::EdgeRef, Direction};

use crate::deps::{DepsGraph, PkgRef};

impl DepsGraph {
    pub fn levels_to_nix(&self, levels: &Vec<Vec<PkgRef>>) -> String {
        let mut all_flakes = self
            .pkgs
            .iter()
            .map(|p| &p.flake_slug)
            .collect::<Vec<&String>>();
        all_flakes.sort();
        all_flakes.dedup();

        let fin = Self::lvl_idx(levels.len() - 1);

        format!(
            "{{ {} ... }}: {}",
            all_flakes.iter().fold(String::new(), |a, b| a + b + ", "),
            nix_let(
                levels.iter().enumerate().map(|(i, lvl)| {
                    (
                        Self::lvl_idx(i),
                        format!(
                            "{} // {}",
                            if i > 0 {
                                Self::lvl_idx(i - 1)
                            } else {
                                "{}".to_string()
                            },
                            nix_attrset(lvl.iter().map(|&pkg_idx| {
                                let pkg = self.get(pkg_idx);

                                (
                                    pkg.name.to_string(),
                                    format!(
                                        "{}.{} {{ {} }}",
                                        pkg.flake_slug,
                                        pkg.name,
                                        self.g
                                            .edges_directed(pkg_idx.into(), Direction::Incoming)
                                            .map(|dep_idx| format!(
                                                "{} = {}.{}",
                                                &self.get(dep_idx.source().index()).name,
                                                fin,
                                                &self.get(dep_idx.source().index()).name
                                            ))
                                            .fold(String::new(), |a, b| format!("{a} {b};"))
                                    ),
                                )
                            }))
                        ),
                    )
                }),
                &fin,
            )
        )
    }

    fn lvl_idx(i: usize) -> String {
        format!("lvl{i}")
    }
}

fn nix_bind(to: &str, value: &str) -> String {
    format!("{} = {};\n", to, value)
}

fn nix_let<'a>(vals: impl Iterator<Item = (String, String)>, body: &'a str) -> String {
    let mut out = "let ".to_string();
    for (to, val) in vals {
        out.push_str(&nix_bind(&to, &val));
        out.push('\n');
    }

    out.push_str("\n in ");
    out.push_str(body);

    out
}

fn nix_attrset<'a>(vals: impl Iterator<Item = (String, String)>) -> String {
    format!(
        "{{\n{}\n}}",
        vals.map(|(to, val)| nix_bind(&to, &val))
            .collect::<String>()
    )
}
