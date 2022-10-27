use std::{path::PathBuf, env};

use anyhow::{anyhow, Context};
use clap::Parser;
use risotto::{
    arguments::{Arguments, RisottoCommand},
    model::Risotto,
};

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    match arguments.subcommand {
        RisottoCommand::Init { path } => {
            let risotto = Risotto::init();

            risotto.dump(path)?;
        }
        RisottoCommand::Apply { path } => {
            let risotto = Risotto::load(&path)?;

            env::set_current_dir(&path.parent().ok_or(anyhow!("could not get the parent of the risotto.toml file"))?).context("could not set the current working directory")?;

            risotto.apply()?;
        }
        RisottoCommand::Add {
            target,
            local,
        } => {
            let path = PathBuf::from("./risotto.toml");

            let mut risotto = if path.exists() {
                Risotto::load(&path)?
            } else {
                Risotto::init()
            };

            risotto.add(target, local)?;

            risotto.dump(&path)?;
        }
    }

    Ok(())
}
