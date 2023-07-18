// The virtual_file library supports creating a virtual file that appears to
// be a contiguous line-based file but which may be represented by multiple
// backing files on disk
//

// a virtual file is a series of backing files, sections that represent a mapping of line numbers
//
//

use std::fs;
// use std::str;

#[derive(Debug)]
struct BackingFile {
    name: String,
    lines: Vec<String>,
    size: usize,
}

// struct Section<'a> {
//     backing: &'a BackingFile,
//     indices: (u32, u32),
// }

// struct VirtualFile {
//     sections: Vec<Section>,
// }

// pub fn new_from_file(name: &str) -> BackingFile {
//     let content = fs::read_to_string("Cargo.toml").expect("Unable to read file");
//     BackingFile {
//         name: String::from(name),
//         lines: content.lines(),
//     }
// }

pub fn new_from_file(name: &str) -> BackingFile {}

fn main() {
    let content = fs::read_to_string("dtrh.rez").expect("Unable to read file!");
    // let lines: Vec<&str> = content.lines().collect();
    let lines: Vec<String> = content.lines().map(|s: &str| String::from(s)).collect();
    let count = lines.len();

    let backing_file = BackingFile {
        name: String::from("dtrh.rez"),
        lines: lines,
        size: count,
    };

    println!("Backing file: {}", backing_file.name);
    println!("Contents: {:?}", backing_file.lines);
    println!("Length: {}", backing_file.size);
}
