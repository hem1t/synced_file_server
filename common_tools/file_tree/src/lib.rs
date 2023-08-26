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
        if path.exists() && path.is_dir() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftree_from_path() {}
}
