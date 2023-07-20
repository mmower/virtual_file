// virtual_file
//
// a virtual file represents a contiguous text file that is, behind the
// scenes, composed of sections of, potentially many, other text files
//
// primary use case is for implementing "include" functionality allowing
// the contents of arbitrary files to be inserted into the text
//

use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct BackingFile {
    name: String,
    lines: Vec<String>,
    size: usize,
}

impl BackingFile {
    pub fn new_from_file(file_name: &str) -> BackingFile {
        let content = fs::read_to_string(file_name).expect("Unable to read file!");
        let lines: Vec<String> = content.lines().map(|s: &str| String::from(s)).collect();
        let count = lines.len();
        BackingFile {
            name: String::from(file_name),
            lines: lines,
            size: count,
        }
    }
}

#[derive(Debug)]
struct Section<'a> {
    backing: &'a BackingFile,
    slice: Range<usize>,
}

impl<'a> Section<'a> {
    pub fn new_from_backing_file(backing_file: &BackingFile) -> Section {
        Section {
            backing: backing_file,
            slice: 0..backing_file.size,
        }
    }
}

#[derive(Debug)]
struct VirtualFile<'a> {
    name: String,
    backing_files: Vec<BackingFile>,
    sections: Vec<Section<'a>>,
}

impl<'a> VirtualFile<'a> {
    pub fn new_from_file(file_name: &str) -> VirtualFile {
        let backing_file = BackingFile::new_from_file(file_name);
        let section = Section::new_from_backing_file(&backing_file);

        VirtualFile {
            name: String::from(file_name),
            backing_files: vec![backing_file],
            sections: vec![section],
        }
    }
}

fn main() {
    let virtual_file = VirtualFile::new_from_file("dtrh.rez");

    println!("Backing file: {:?}", virtual_file.name);
}
