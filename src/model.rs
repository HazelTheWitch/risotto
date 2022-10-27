use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use symlink::{remove_symlink_auto, symlink_auto};

#[derive(Debug, Serialize, Deserialize)]
pub struct Risotto {
    #[serde(rename = "config")]
    pub configs: Option<Vec<Config>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub source: PathBuf,
    pub target: PathBuf,
}

fn is_symlink<P: AsRef<Path>>(path: P) -> anyhow::Result<bool> {
    let metadata = fs::symlink_metadata(path)?;
    Ok(metadata.file_type().is_symlink())
}

impl Config {
    pub fn link(&self) -> anyhow::Result<()> {
        if self.target.exists() {
            if is_symlink(&self.target).context("could not check if target is a symlink")? {
                remove_symlink_auto(&self.target)?;
            } else {
                if self.target.is_file() {
                    fs::remove_file(&self.target)?;
                } else {
                    fs::remove_dir_all(&self.target)?;
                }
            }
        }

        symlink_auto(
            &self
                .source
                .canonicalize()
                .context("could not canonicalize source")?,
            &self.target,
        )
        .context("could not symlink")?;

        Ok(())
    }
}

impl Risotto {
    pub fn init() -> Self {
        Self { configs: None }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let data = fs::read(&path).context(format!("could not read `{}`", path.as_ref().to_string_lossy()))?;
        Ok(toml::from_slice(&data).context("invalid risotto.toml file")?)
    }

    pub fn dump<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self).context("could not serialize risotto")?;
        fs::write(&path, data).context(format!("could not write to `{}`", path.as_ref().to_string_lossy()))?;

        Ok(())
    }

    pub fn apply(&self) -> anyhow::Result<()> {
        for config in self.configs.as_ref().unwrap_or(&vec![]) {
            config.link()?;

            println!(
                "{} -> {}",
                config.source.to_string_lossy(),
                config.target.to_string_lossy()
            );
        }

        Ok(())
    }

    pub fn add(&mut self, target: PathBuf, local: PathBuf) -> anyhow::Result<()> {
        let config = Config {
            source: local,
            target,
        };

        match self.configs.as_mut() {
            Some(configs) => {
                configs.push(config);
            }
            None => {
                self.configs = Some(vec![config]);
            }
        }

        Ok(())
    }
}
