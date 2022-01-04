use std::{env::args, ffi::OsStr, fs::read_dir};

use anyhow::Result;
use bincode::{deserialize, serialize};
use letsroleffmusic::{db, BIt, IgnoreExt, OrNothingExt, SingleExt, Song, ALBUM_RE, SONGS_TREE};
use regex::Regex;

fn main() -> Result<()> {
    let mut args: BIt<String> = Box::new(args().skip(1));
    let dir_str = args.single()?;
    let tree = db()?.open_tree(SONGS_TREE)?;
    let last_key = {
        if let Some((k, _)) = tree.last()? {
            deserialize::<u64>(&k)?
        } else {
            0
        }
    };
    let title_re = Regex::new(r"[^a-z0-9 ]")?;
    let album_re = Regex::new(ALBUM_RE)?;
    for (i, f) in read_dir(dir_str)?.enumerate() {
        let f = f?.path();
        if f.extension() != Some(OsStr::new("m4a")) {
            continue;
        }
        let file = taglib::File::new(&f).ignore()?;
        let tag = file.tag().ignore()?;
        let title_raw = tag.title().or_nothing()?.to_lowercase();
        let title = title_re.replace_all(&title_raw, "").to_string();
        let album_raw = tag.album().or_nothing()?;
        let album = album_re.captures(&album_raw).or_nothing()?[1]
            .to_owned()
            .to_lowercase();
        let key = last_key + i as u64 + 1;
        let song = Song {
            title,
            album,
            found_video: None,
        };
        tree.insert(serialize(&key)?, serialize(&song)?)?;
    }
    Ok(())
}
