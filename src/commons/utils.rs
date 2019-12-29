use std::path::PathBuf;

pub fn get_output_file(output_dir: &PathBuf, input_file: &PathBuf) -> PathBuf {
    let file_name = input_file.file_name().unwrap();
    let mut output_dir = output_dir.to_owned();
    output_dir.push(PathBuf::from(file_name));
    output_dir.with_extension("png")
}