use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn list_paths(path: PathBuf) -> Vec<PathBuf> {
    let reader = BufReader::new(File::open(path).expect("Counldn't open the Config"));
    reader
        .lines()
        .map(|line| {
            let path = PathBuf::from(resolve_envs(line.unwrap()));
            if !path.exists() {
                eprintln!("Warning Path: {:?}, does not exists.", path);
            }
            path
        })
        .collect()
}

fn resolve_envs(path: String) -> String {
    let new_path = path
        .split('/')
        .into_iter()
        .map(|name| {
            if name.is_empty() {
                String::new()
            } else if name.starts_with("$") {
                env::var(name.get(1..).unwrap()).unwrap()
            } else {
                format!("/{}", name)
            }
        })
        .collect::<String>();
    new_path
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_file_paths_from_config() {
        let config_path = format!("{}/.config", env::var("HOME").unwrap());
        assert_eq!(
            vec![
                PathBuf::from(config_path),
                PathBuf::from("/home/user/.local"),
                PathBuf::from("/hlkj/ljoi/lj"),
            ],
            list_paths(PathBuf::from("config_paths_example"))
        );
    }
}
