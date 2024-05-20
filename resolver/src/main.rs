use anyhow::Result;

use crate::locator::LambdaLocs;
mod locator;

fn main() -> Result<()> {
    let locs = LambdaLocs::for_flake_spec("..#core")?;

    dbg!(locs);

    Ok(())
}
