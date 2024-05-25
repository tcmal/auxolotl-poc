use std::fmt::{self, Display, Formatter};

/// A package, with location information
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Package {
    /// The name of the attribute it is stored under
    pub name: String,

    /// The position of the lambda which returns the derivation
    pub pos: SourcePos,

    /// Slug describing the flake this came from.
    pub flake_slug: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Identififes a position in nix code
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SourcePos {
    pub file: String,
    pub row: usize,
    pub col: usize,
}

impl SourcePos {
    /// Convert to an offset in bytes within the given file.
    /// Returns None if the position is out of bounds.
    pub fn to_offset(&self, mut contents: &str) -> Option<usize> {
        let mut pos = 0;
        let mut skip_rows = self.row - 1;
        while skip_rows > 0 {
            let r = contents.split_once('\n')?;
            pos += r.0.len() + 1;
            contents = r.1;
            skip_rows -= 1;
        }

        Some(pos + self.col - 1)
    }
}
