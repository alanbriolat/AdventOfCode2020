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
