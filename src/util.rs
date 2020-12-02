use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[macro_export]
macro_rules! data_path {
    ($filename:expr) => {{
        use std::env;
        use std::path::PathBuf;
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = PathBuf::from(root_dir);
        source_path.push("data");
        source_path.push($filename);
        source_path
    }};
}

pub fn read_lines(path: &PathBuf) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map(Result::unwrap)
}

pub fn str_partition<'a>(input: &'a str, sep: &str) -> (&'a str, &'a str) {
    match input.find(sep) {
        Some(pos) => (&input[..pos], &input[(pos + sep.len())..]),
        None => (input, ""),
    }
}
