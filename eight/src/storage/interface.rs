use anyhow::{Error, Result};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Default)]
pub struct Storage {
    pub(crate) path: PathBuf,
}

impl FromStr for Storage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            path: PathBuf::from_str(s)?,
            ..Default::default()
        })
    }
}
