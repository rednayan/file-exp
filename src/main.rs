use std::{env, fs};

fn main() {
    let mut arg_vec = Vec::new();
    for args in env::args() {
        arg_vec.push(args);
    }
    let path = String::from(&arg_vec[1]);

    let directories = fs::read_dir(&path).unwrap();
    for entry in directories {
        println!("{:?}", entry.unwrap());
    }
}
