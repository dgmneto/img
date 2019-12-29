use std::path::PathBuf;
use structopt::StructOpt;

use crate::commons::image::Image;
use crate::commons::utils::get_output_file;
use crate::sort::{Sort, SortReturn};

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct SortOpt {
    /// Images to be processed
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,

    /// Images output directory
    #[structopt(short = "O", long = "output")]
    output_dir: PathBuf,

    /// Fractions of rows to sort
    #[structopt(short = "r", long = "rows_to_sort", default_value = "0.15")]
    rows_to_sort: f64,
}

pub fn execute_sort(sort_opt: SortOpt) -> Result<(),failure::Error> {
    let SortOpt{files, output_dir, rows_to_sort} = sort_opt;
    for file in files {
        let image = Image::from_file(&file)?;
        let sort = Sort::new(image, rows_to_sort);
        let SortReturn {image, ..} = sort.run();
        let output_file = get_output_file(&output_dir, &file);
        image.save_to_path(&output_file)?;
    }
    Ok(())
}