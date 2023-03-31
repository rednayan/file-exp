use std::{
    env,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

fn main() {
    let mut arg_vec = Vec::new();
    for args in env::args() {
        arg_vec.push(args);
    }
    let path = String::from(&arg_vec[1]);
    visit_dir(&PathBuf::from(&path));
}

fn visit_dir(path: &Path) {
    let directories = fs::read_dir(path).unwrap();
    for entry in directories {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            println!("subpath: {:?}", &subpath);
            visit_dir(&subpath);
        }
        let metadata: Metadata = fs::metadata(&subpath).unwrap();
        // println!("{:?}", subpath);
    }
}
