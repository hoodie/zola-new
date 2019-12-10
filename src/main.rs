use structopt::StructOpt;

mod opt;
mod front_matter;

use self::opt::{App, Command};
use self::front_matter::Page;

use std::{
    convert::TryInto,
    fs,
    io::Write,
    path::{Path, PathBuf},
};


fn create_page(target_folder: &Path) -> std::result::Result<PathBuf, std::io::Error>{
    if dbg!(target_folder.exists()) {
        Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists))
    } else {
        fs::create_dir(target_folder)?;
        let target_file = dbg!(target_folder.join("index.md"));
        Ok(dbg!(target_file))
    }
}

fn write_page(target_file: &Path, content: &str) {
    if dbg!(target_file.exists()) {
        println!("{:?} already exists, can't overwrite", target_file);
    } else if let Ok(mut file) = fs::File::create(target_file) {
        for line in content.lines() {
            file.write_fmt(format_args!("{}\n", line))
                .expect("cannot write this line to the config file");
        }
    }
}


fn main() {
    color_backtrace::install();
    let config = match App::from_args().command {
        Command::Page(config) => config,
        Command::Post(config) => config,
    };
    let page = Page::from(config);

    let path = page.file_name();
    let content: String = page.try_into().unwrap();

    println!("{}", content);
    println!("<!-- {} -->", path);
}