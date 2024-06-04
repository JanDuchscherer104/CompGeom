use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_dat_paths(data_dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut dat_files = Vec::new();
    for entry in fs::read_dir(Path::new(data_dir))? {
        let entry = entry?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("dat") {
            dat_files.push(entry.path());
        }
    }
    println!("Found {} .dat files", dat_files.len());
    println!("{:?}", dat_files);
    Ok(dat_files)
}
