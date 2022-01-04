use std::time::Duration;

use anyhow::Result;
use bincode::deserialize;
use governor::{Jitter, Quota, RateLimiter};
use letsroleffmusic::{db, OrNothingExt, Song, SEARCH_PREFIX, SONGS_TREE};
use youtube_dl::{YoutubeDl, SearchOptions, YoutubeDlOutput};

#[tokio::main]
async fn main() -> Result<()> {
    let quota = Quota::with_period(Duration::from_secs(2)).or_nothing()?;
    let lim = RateLimiter::direct(quota);
    let jitter = Jitter::up_to(Duration::from_millis(1900));
    let tree = db()?.open_tree(SONGS_TREE)?;
    // !!! debug
    let mut single = false;
    // !!! debug
    for r in tree.range::<&[u8], _>(..) {
        // !!! debug
        if single { break; }
        single = true;
        // !!! debug
        lim.until_ready_with_jitter(jitter).await;
        let song = deserialize::<Song>(&r?.1)?;
        let query = format!("{} {} {}", SEARCH_PREFIX, song.album, song.title);
        let out = YoutubeDl::search_for(&SearchOptions::youtube(query)).run()?;
        let url = match out {
            YoutubeDlOutput::Playlist(a) => {
                dbg!(&a);
                let vec = a.entries.or_nothing()?;
                let first = vec.first().or_nothing()?;
                first.url.clone()
            },
            YoutubeDlOutput::SingleVideo(a) => {
                dbg!(&a);
                a.url
            },
        }.or_nothing()?;
        println!("{}", url);
    }
    Ok(())
}
