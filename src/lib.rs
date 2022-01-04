use directories::UserDirs;
use serde::{Serialize, Deserialize};
use sled::Db;
use thiserror::Error;
use anyhow::{Result, bail};

#[derive(Error, Debug)]
#[error("continuation on `None`")]
pub struct Nothing;

const DBNAME: &str = "letsroleffmusic.sled";
pub const SEARCH_PREFIX: &str = "ffxiv ost";
pub const SONGS_TREE: &str = "songs";
pub const VIDEOS_TREE: &str = "videos";
pub const TREES: &[&str] = &[SONGS_TREE];
pub const ALBUM_RE: &str = r"^([^:]*):? FINAL FANTASY(.*)$";

pub fn db() -> Result<Db> {
    let directories_ud = UserDirs::new().ok_or(Nothing)?;
    let desktop = directories_ud.desktop_dir().ok_or(Nothing)?;
    let db = sled::open(desktop.join(DBNAME))?;
    Ok(db)
}

pub type BIt<T> = Box<dyn Iterator<Item = T>>;

pub trait OrNothingExt<T> {
    fn or_nothing(self) -> Result<T>;
}

impl<T> OrNothingExt<T> for Option<T> {
    fn or_nothing(self) -> Result<T> {
        Ok(self.ok_or(Nothing)?)
    }
}

pub trait IgnoreExt<T, E> {
    fn ignore(self) -> Result<T>;
}

impl<T, E> IgnoreExt<T, E> for Result<T, E> {
    fn ignore(self) -> Result<T> {
        Ok(self.map_err(|_| Nothing)?)
    }
}

pub trait SingleExt<T> {
    fn single(&mut self) -> Result<T>;
}

impl<T> SingleExt<T> for dyn Iterator<Item = T> {
    fn single(&mut self) -> Result<T> {
        let first = self.next().or_nothing()?;
        if let Some(_) = self.next() {
            bail!("");
        }
        Ok(first)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub album: String,
    pub found_video: Option<String>,
}
