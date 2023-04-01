use anyhow::Result;
use std::{
    env,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let mut arg_vec = Vec::new();
    for args in env::args() {
        arg_vec.push(args);
    }
    let path = String::from(&arg_vec[1]);
    visit_dir(&PathBuf::from(&path))?;
    Ok(())
}

fn visit_dir(path: &Path) -> Result<()> {
    let directories = fs::read_dir(path)?;
    for entry in directories {
        let entry = entry?;
        let subpath = entry.path();
        if subpath.is_dir() {
            println!("subpath: {:?}", &subpath);
            visit_dir(&subpath)?;
        }
        let metadata: Metadata = fs::metadata(&subpath)?;
        // println!("{:?}", subpath);
    }
    Ok(())
}
