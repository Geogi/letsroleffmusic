use anyhow::Result;
use letsroleffmusic::{db, TREES};

fn main() -> Result<()> {
    let db = db()?;
    for tree in TREES {
        db.drop_tree(tree)?;
    }
    db.clear()?;
    Ok(())
}
