use ignore::Walk;
use std::io::Write;
use zip::write::{FileOptions, ZipWriter};

pub fn zip_dir(path: &str, zip_file_name: &str) {
    let mut zip = ZipWriter::new(std::fs::File::create(zip_file_name).unwrap());
    for entry in Walk::new(path) {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap() == zip_file_name {
            continue;
        }
        if entry.path().is_dir() {
            zip.add_directory(entry.path().to_str().unwrap(), FileOptions::default());
        } else {
            zip.start_file_from_path(&entry.path(), FileOptions::default())
                .unwrap();
            zip.write_all(std::fs::read(entry.path()).unwrap().as_slice())
                .unwrap();
        }
    }
}
