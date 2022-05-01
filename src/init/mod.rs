use rust_embed::RustEmbed;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "src/init/static"]
#[exclude = ".DS_Store"]
struct Asset;

pub fn run(path: &str) {
    println!("Initialize a new Orion project in {}", path);

    if Path::new(path).exists() {
        println!("Path \"{}\" already exists, please chose another one", path);
        return;
    }

    fs::create_dir_all(path).unwrap();
    fs::create_dir_all(format!("{}/articles", path)).unwrap();
    fs::create_dir_all(format!("{}/themes/orion", path)).unwrap();

    for file in Asset::iter() {
        let file_path = format!("{}/{}", path, file);
        println!("Creating {}", file_path);

        let asset = Asset::get(&file).unwrap();
        let mut f = File::create(Path::new(file_path.as_str())).unwrap();
        f.write_all(&asset.data).unwrap();
    }
}
