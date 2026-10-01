// mochou-p/sauce/src/clean.rs

pub fn command() {
    let path = format!("{}{}/", super::ROOT, env!("CARGO_BIN_NAME"));

    std::fs::remove_dir_all(path).unwrap();
}

