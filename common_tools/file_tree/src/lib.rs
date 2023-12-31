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

// This impl should hold private fns
impl FTree {
    fn has_children(&self) -> bool {
        !&self.children.is_empty()
    }
}

#[allow(dead_code)]
impl FTree {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            children: Vec::new(),
        }
    }

    ///
    /// Purpose: To create `FTree` recursively.
    ///
    pub fn from_path(path: PathBuf) -> Result<Self, FTreeErr> {
        let mut root = Self::new(path.clone());
        if path.exists() {
            if path.is_dir() {
                let entries = path.read_dir();
                if entries.is_err() {
                    return Err(FTreeErr::NotReadable(path.clone()));
                }

                for entry in entries.unwrap() {
                    if let Ok(entry) = entry {
                        let child = Self::from_path(entry.path());
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

    ///
    /// Purpose: To create `FTree` recursively, out of string.
    ///
    pub fn from_string(tree: String) -> Result<Self, FTreeErr> {
        todo!()
    }

    ///
    /// Purpose: To return a time for the file or directory to check if
    ///          it was recently modified.
    ///
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

    ///
    /// Purpose: To return unique string for the FTree Structure,
    ///          which will have unique hash value, when calculated.
    ///
    pub fn as_string(&self) -> Result<String, FTreeErr> {
        let mut tree = String::from(self.path.to_str().unwrap());
        tree.push(':');
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

    ///
    /// Purpose: Create file structure using `FTree`.
    ///
    pub fn write_to_path(_path: PathBuf) -> Result<(), FTreeErr> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftree_from_path() {
        pretty_assertions::assert_eq!(
            "./examples_dir:1703885422,./examples_dir/test_dir:1703885422,./examples_dir/test_dir/second_file:1703885422;./examples_dir/test_dir/a_dir:1703885422,./examples_dir/test_dir/a_dir/with_file:1703885422;./examples_dir/test_dir/hello_file:1704060920;",
            FTree::from_path(PathBuf::from("./examples_dir"))
                .unwrap()
                .as_string()
                .unwrap()
        );
    }
}
