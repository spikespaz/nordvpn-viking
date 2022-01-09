use build_helpers::prelude::*;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest: Manifest<Metadata> = Manifest::from_path_with_metadata("Cargo.toml").unwrap();

    println!("Output directory: {:?}", out_dir);
    // println!("Manifest: {:#?}", manifest);

    for (name, detail) in manifest
        .package
        .unwrap()
        .metadata
        .unwrap()
        .foreign_dependencies
        .into_iter()
    {
        let updated = detail.update(&out_dir);
        if !updated {
            continue;
        }

        println!("Updated foreign dependency: {}", name);
    }

    panic!();
}
