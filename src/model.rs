use std::{path::PathBuf, fs, os::unix::fs::symlink, ffi::OsString};

use anyhow::Context;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Risotto {
    #[serde(rename="config")]
    pub configs: Option<Vec<Config>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub source: PathBuf,
    pub target: PathBuf,
    pub symbolic: Option<bool>,
}

impl Config {
    pub fn backup(&self) -> anyhow::Result<()> {
        let extension = {
            match self.target.extension() {
                Some(extension) => {
                    let mut extension = extension.to_os_string();
                    extension.push(".bak");
                    extension
                },
                None => OsString::from("bak"),
            }
        };

        let backup_path = self.target.with_extension(extension);

        fs::write(&backup_path, fs::read(&self.target)
                .context(format!("could not read `{}`", self.target.to_string_lossy()))?)
            .context(format!("could not write `{}`", backup_path.to_string_lossy()))?;
        fs::remove_file(&self.target)?;

        Ok(())
    }

    pub fn link(&self) -> anyhow::Result<()> {
        if self.symbolic.unwrap_or(true) {
            symlink(
                &self.source.canonicalize().context("could not canonicalize source")?,
                &self.target
            ).context("could not symlink")?;
        } else {
            fs::copy(&self.source, &self.target)?;
        }

        Ok(())
    }
}

impl Risotto {
    pub fn init() -> Self {
        Self { configs: None }
    }

    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let data = fs::read(&path).with_context(|| format!("could not read `{:?}`", path))?;
        Ok(toml::from_slice(&data).context("invalid risotto.toml file")?)
    }

    pub fn dump(&self, path: PathBuf) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self).context("could not serialize risotto.toml")?;
        fs::write(path, data).context("could not write to risotto.toml")?;

        Ok(())
    }

    pub fn apply(&self, backup: bool) -> anyhow::Result<()> {
        for config in self.configs.as_ref().unwrap_or(&vec![]) {
            if backup {
                config.backup()?;
            }

            config.link()?;

            println!("{} -> {}", config.source.to_string_lossy(), config.target.to_string_lossy());
        }

        Ok(())
    }
}
