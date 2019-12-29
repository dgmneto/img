use quicli::prelude::CliResult;
use structopt::StructOpt;

mod cli;
mod commons;
mod k_means;
mod sort;

use cli::{Opt, OptCommand};
use cli::{execute_k_means, execute_sort};

fn main() -> CliResult {
    let args = Opt::from_args();
    args.verbosity.setup_env_logger("img")?;
    match args.command {
        OptCommand::KMeans(k_means_opt) => execute_k_means(k_means_opt)?,
        OptCommand::Sort(sort_opt) => execute_sort(sort_opt)?,
    }
    Ok(())
}
