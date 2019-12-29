use std::path::PathBuf;
use structopt::StructOpt;

use crate::commons::image::Image;
use crate::commons::utils::get_output_file;
use crate::k_means::{KMeans, KMeansReturn};

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct KMeansOpt {
    /// Number of means to use. Results in the final number of possible colors.
    #[structopt(short = "k")]
    k_means: u8,

    /// Images to be processed
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,

    /// Images output directory
    #[structopt(short = "O", long = "output")]
    output_dir: PathBuf,

    /// Shoul deduplicate visually simillar colors' means
    #[structopt(short = "d", long = "deduplicate")]
    deduplicate: bool,
}

pub fn execute_k_means(k_means_opt: KMeansOpt) -> Result<(),failure::Error> {
    let KMeansOpt{k_means, files, output_dir, deduplicate} = k_means_opt;
    for file in files {
        let image = Image::from_file(&file)?;
        let k_means = KMeans::new(k_means, deduplicate, image);
        let KMeansReturn {image, ..} = k_means.run();
        let output_file = get_output_file(&output_dir, &file);
        image.save_to_path(&output_file)?;
    }
    Ok(())
}