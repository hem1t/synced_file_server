use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn list_paths(path: PathBuf) -> Vec<PathBuf> {
    let reader = BufReader::new(File::open(path).expect("Counldn't open the Config"));
    reader
        .lines()
        .map(|line| {
            let path = PathBuf::from(line.unwrap());
            if !path.exists() {
                eprintln!("Warning Path: {:?}, does not exists.", path);
            }
            path
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_paths_from_config() {
        assert_eq!(
            vec![
                PathBuf::from("/home/user/.config"),
                PathBuf::from("/home/user/.local"),
                PathBuf::from("/hlkj/ljoi/lj"),
            ],
            list_paths(PathBuf::from("config_paths_example"))
        );
    }
}
