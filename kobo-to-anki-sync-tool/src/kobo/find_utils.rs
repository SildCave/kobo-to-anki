use std::path::PathBuf;

pub fn find_and_validate_kobo_path() -> Option<PathBuf> {
    let list = disk_list::get_disk_list();
    for disk in &list {
        if disk.get(2).unwrap().contains("KOBOeReader") {
            //println!("{:?}", list);
            let kobo_path = PathBuf::from(disk.get(2).unwrap());
            if validate_kobo_path(&kobo_path) {
                return Some(kobo_path);
            }
        }
    }
    None
}

pub fn validate_kobo_path(path: &PathBuf) -> bool {
    let kobo_reader_sqlite = path.join(".kobo/KoboReader.sqlite");
    if kobo_reader_sqlite.exists() {
        return true;
    }
    false
}