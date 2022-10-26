use std::path::PathBuf;

use clap::Parser;
use risotto::{arguments::{Arguments, RisottoCommand}, model::Risotto};

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    match arguments.subcommand {
        RisottoCommand::Init { path } => {
            let risotto = Risotto::init();

            risotto.dump(path)?;
        },
        RisottoCommand::Apply => {
            let risotto = Risotto::load(PathBuf::from("./risotto.toml"))?;

            risotto.apply()?;
        },
        RisottoCommand::Add { target, local, symbolic } => {
            let path = PathBuf::from("./risotto.toml");
            
            let mut risotto = if path.exists() { Risotto::load(path.clone())? } else { Risotto::init() };

            risotto.add(target, local, symbolic)?;

            risotto.dump(path.clone())?;
        },
    }

    Ok(())
}
