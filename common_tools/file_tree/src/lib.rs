use std::{
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

pub struct FTree {
    path: PathBuf,
    children: Vec<Box<FTree>>,
}

impl FTree {
    fn new(path: PathBuf) -> Self {
        FTree {
            path,
            children: Vec::new(),
        }
    }

    pub fn from_path(path: PathBuf) -> Result<FTree, String> {
        let mut root = FTree::new(path.clone());
        if path.exists() {
            if path.is_dir() {
                let entries = path.read_dir();
                if entries.is_err() {
                    return Err(format!("Can't read {:?}", path));
                }

                for entry in entries.unwrap() {
                    if let Ok(entry) = entry {
                        let child = FTree::from_path(entry.path());
                        if let Ok(child) = child {
                            root.children.push(Box::new(child));
                        } else {
                            return child;
                        }
                    } else {
                        return Err(format!("{:?} broken at: {:?}", entry, path));
                    }
                }
            }
            return Ok(root);
        } else {
            Err(format!("Path {:?} does not exists", path))
        }
    }

    pub fn is_dir(&self) -> bool {
        return self.path.is_dir();
    }

    pub fn is_file(&self) -> bool {
        return self.path.is_file();
    }

    pub fn as_string(&self) -> String {
        let mut tree = String::from(self.path.to_str().unwrap());
        tree.push_str(&self.file_time().as_secs().to_string().as_str());
        tree.push(',');

        if self.has_children() {
            for child in &self.children {
                tree.push_str(&child.as_string());
            }
        }
        return tree;
    }

    fn has_children(&self) -> bool {
        !&self.children.is_empty()
    }

    fn file_time(&self) -> Duration {
        if let Ok(time) = fs::metadata(self.path.clone())
            .expect(format!("can't access {:?}", self.path).as_str())
            .modified()
        {
            return time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        }
        eprintln!(
            "[Warning] Your platform doesn't supports File Time, so, considering file as new!"
        );
        return SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftree_from_path() {
        assert_eq!(
            "./examples_dir1703859968,./examples_dir/test_dir1703859968,./examples_dir/test_dir/a_dir1703859968,./examples_dir/test_dir/a_dir/with_file1703859968,./examples_dir/test_dir/hello_file1703859968,./examples_dir/test_dir/second_file1703859968,",
            FTree::from_path(PathBuf::from("./examples_dir"))
                .unwrap()
                .as_string()
        );
    }
}
