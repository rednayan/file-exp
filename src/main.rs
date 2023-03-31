use std::{
    env, fs,
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
            visit_dir(&subpath);
        }
        println!("{:?}", subpath);
    }
}
