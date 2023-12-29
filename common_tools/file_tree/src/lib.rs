use std::{
    fs, io,
    path::PathBuf,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
pub enum FTreeErr {
    NotExists(PathBuf),
    NotReadable(PathBuf),
    NotWritable(PathBuf),
    IOErr(io::Error),
}

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

    pub fn from_path(path: PathBuf) -> Result<FTree, FTreeErr> {
        let mut root = FTree::new(path.clone());
        if path.exists() {
            if path.is_dir() {
                let entries = path.read_dir();
                if entries.is_err() {
                    return Err(FTreeErr::NotReadable(path.clone()));
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
                        return Err(FTreeErr::IOErr(entry.err().unwrap()));
                    }
                }
            }
            return Ok(root);
        } else {
            return Err(FTreeErr::NotExists(path.clone()));
        }
    }

    pub fn is_dir(&self) -> bool {
        return self.path.is_dir();
    }

    pub fn is_file(&self) -> bool {
        return self.path.is_file();
    }

    pub fn as_string(&self) -> Result<String, FTreeErr> {
        let mut tree = String::from(self.path.to_str().unwrap());
        tree.push_str(&self.file_time()?.as_secs().to_string().as_str());

        if self.has_children() {
            tree.push(',');
            for child in &self.children {
                tree.push_str(&child.as_string()?);
            }
        } else {
            tree.push(';');
        }
        return Ok(tree);
    }

    fn has_children(&self) -> bool {
        !&self.children.is_empty()
    }

    fn file_time(&self) -> Result<Duration, FTreeErr> {
        if let Ok(meta) = fs::metadata(self.path.clone()) {
            if let Ok(time) = meta.modified() {
                return Ok(time.duration_since(SystemTime::UNIX_EPOCH).unwrap());
            } else {
                eprintln!(
                    "[Warning] Your platform doesn't supports File Time, so, considering file as new!"
                );
                return Ok(SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap());
            }
        }
        return Err(FTreeErr::NotReadable(self.path.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftree_from_path() {
        assert_eq!(
            "./examples_dir1703859968,./examples_dir/test_dir1703859968,./examples_dir/test_dir/a_dir1703859968,./examples_dir/test_dir/a_dir/with_file1703859968;./examples_dir/test_dir/hello_file1703859968;./examples_dir/test_dir/second_file1703859968;",
            FTree::from_path(PathBuf::from("./examples_dir"))
                .unwrap()
                .as_string()
                .unwrap()
        );
    }
}
