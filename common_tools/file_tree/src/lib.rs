use std::path::PathBuf;

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
        if self.has_children() {
            for child in &self.children {
                tree.push_str(&child.as_string());
                tree.push(',');
            }
        }
        return tree;
    }

    fn has_children(&self) -> bool {
        !&self.children.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftree_from_path() {
        assert_eq!(
            "./examples_dir./examples_dir/empty_dir,./examples_dir/test_dir./examples_dir/test_dir/a_dir./examples_dir/test_dir/a_dir/with_file,,./examples_dir/test_dir/hello_file,./examples_dir/test_dir/second_file,,",
            FTree::from_path(PathBuf::from("./examples_dir"))
                .unwrap()
                .as_string()
        );
    }
}
