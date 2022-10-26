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
        RisottoCommand::Apply { no_backup } => {
            let risotto = Risotto::load(PathBuf::from("./risotto.toml"))?;

            risotto.apply(!no_backup)?;
        }
    }

    Ok(())
}
