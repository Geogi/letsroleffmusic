use std::time::Duration;

use anyhow::Result;
use bincode::deserialize;
use governor::{Jitter, Quota, RateLimiter};
use letsroleffmusic::{db, OrNothingExt, Song, SEARCH_PREFIX, SONGS_TREE};

#[tokio::main]
async fn main() -> Result<()> {
    let quota = Quota::with_period(Duration::from_secs(2)).or_nothing()?;
    let lim = RateLimiter::direct(quota);
    let jitter = Jitter::up_to(Duration::from_millis(1900));
    let tree = db()?.open_tree(SONGS_TREE)?;
    for r in tree.range::<&[u8], _>(..) {
        lim.until_ready_with_jitter(jitter).await;
        let song = deserialize::<Song>(&r?.1)?;
        println!("search: {} {} {}", SEARCH_PREFIX, song.album, song.title)
    }
    Ok(())
}
