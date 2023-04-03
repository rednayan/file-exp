use anyhow::Result;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct Config {
    path: String,
}

fn main() -> Result<()> {
    let path_config: Config = get_arguments()?;
    visit_dir(&PathBuf::from(&path_config.path))?;
    Ok(())
}

fn get_arguments() -> Result<Config> {
    let mut arg_vec = Vec::new();
    for args in env::args() {
        arg_vec.push(args);
    }
    let path = String::from(&arg_vec[1]);
    let path_config: Config = Config { path };
    Ok(path_config)
}

fn visit_dir(path: &Path) -> Result<()> {
    let directories = fs::read_dir(path)?;
    for entry in directories {
        let entry = entry?;
        let subpath = entry.path();
        if subpath.is_dir() {
            visit_dir(&subpath)?;
        }
        let file_bytes = fs::read(&subpath)?;
        println!("{:?}", file_bytes);
    }
    Ok(())
}
