use std::fs::DirEntry;
use walkdir::WalkDir;

pub(crate) fn is_file(x: &DirEntry) -> bool {
    match x.file_type() {
        Ok(v) => v.is_file(),
        Err(_e) => false,
    }
}

pub fn get_md_files(path: Option<&str>) -> Option<Vec<walkdir::DirEntry>> {
    match path {
        Some(p) => {
            let wd = WalkDir::new(p)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok());

            let mut r: Vec<walkdir::DirEntry> = Vec::new();

            for entry in wd {
                let f_name = entry.file_name().to_string_lossy();
                if f_name.ends_with(".md") {
                    r.push(entry)
                }
            }
            Some(r)
        }
        None => None,
    }
}
