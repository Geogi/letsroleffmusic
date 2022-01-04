use anyhow::Result;
use bincode::deserialize;
use letsroleffmusic::{db, Song, SONGS_TREE};

fn main() -> Result<()> {
    let tree = db()?.open_tree(SONGS_TREE)?;
    for r in tree.range::<&[u8], _>(..) {
        let (k, v) = r?;
        let k = deserialize::<u64>(&k)?;
        let song = deserialize::<Song>(&v)?;
        println!("{}: {}", k, song.title)
    }
    Ok(())
}
