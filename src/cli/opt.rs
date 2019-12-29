use quicli::prelude::Verbosity;
use structopt::StructOpt;

use super::k_means::KMeansOpt;
use super::sort::SortOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "img")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: OptCommand,

    #[structopt(flatten)]
    pub verbosity: Verbosity,
}

#[derive(StructOpt, Debug)]
#[structopt()]
pub enum OptCommand {
    /// Execute K Means compression strategy in image
    #[structopt(name = "kmeans")]
    KMeans(KMeansOpt),

    /// Execute K Means compression strategy in image
    #[structopt(name = "sort")]
    Sort(SortOpt),
}