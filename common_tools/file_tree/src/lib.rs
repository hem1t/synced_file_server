#![allow(unused, dead_code)]
use std::{
    fs, io,
    path::PathBuf,
    str::FromStr,
    time::{Duration, SystemTime},
};

#[derive(Clone)]
pub struct FNode {
    path: PathBuf,
    is_f: bool,
    modified: Duration,
}

// This impl should hold private fns
impl FNode {
    fn new(path: &PathBuf) -> Result<Self, io::Error> {
        Ok(Self {
            path: path.clone(),
            is_f: path.is_file(),
            modified: FNode::node_time(path)?,
        })
    }

    ///
    /// Purpose: To return a time for the file or directory to check if
    ///          it was recently modified.
    ///
    fn node_time(path: &PathBuf) -> Result<Duration, io::Error> {
        let meta = fs::metadata(path.clone())?;
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

    fn to_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.path.to_string_lossy(),
            self.modified.as_secs(),
            if self.is_f { 'f' } else { 'd' }
        )
    }

    fn from_string(s: String) -> Result<Self, io::Error> {
        let mut n = s.split(':');
        Ok(Self {
            path: PathBuf::from_str(n.next().unwrap()).unwrap(),
            modified: Duration::from_secs(n.next().unwrap().parse::<u64>().unwrap()),
            is_f: n.next().unwrap() == "f",
        })
    }
}

struct FTree {
    nodes: Vec<FNode>,
    // largest avail Duration.
    modified: Duration,
}

impl FTree {
    pub fn new(path: PathBuf) -> Result<FTree, io::Error> {
        let mut ftree = Self {
            nodes: vec![FNode::new(&path)?],
            modified: Duration::from_secs(0),
        };
        ftree.from_path(path)?;
        Ok(ftree)
    }

    fn add(&mut self, node: FNode) {
        self.nodes.push(node.clone());
        // larger modified time;
        if self.modified < node.modified {
            self.modified = node.modified;
        }
    }

    ///
    /// Purpose: To list all file and dirs
    ///
    fn from_path(&mut self, path: PathBuf) -> Result<(), io::Error> {
        let entries = path.read_dir()?;
        for entry in entries {
            if let Ok(e) = entry {
                let e = e.path();
                self.add(FNode::new(&e)?);
                if e.is_dir() {
                    self.from_path(e)?;
                }
            } else {
                return Err(entry.err().unwrap());
            }
        }
        Ok(())
    }

    ///
    /// Purpose: To return unique string for the FTree Structure,
    ///          which will have unique hash value, when calculated.
    ///
    pub fn as_string(&self) -> Result<String, io::Error> {
        Ok(self
            .nodes
            .iter()
            .map(|n| format!("{},", n.to_string()))
            .collect::<String>())
    }

    pub fn from_string(s: String) -> Result<FTree, io::Error> {
        let mut ftree = FTree {
            nodes: Vec::new(),
            modified: Duration::from_secs(0),
        };
        for node in s.split(',') {
            ftree.add(FNode::from_string(node.to_string())?);
        }
        Ok(ftree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftree_from_path() {
        pretty_assertions::assert_eq!(
            "./examples_dir:1703885422:d,./examples_dir/test_dir:1703885422:d,./examples_dir/test_dir/second_file:1703885422:f,./examples_dir/test_dir/a_dir:1703885422:d,./examples_dir/test_dir/a_dir/with_file:1703885422:f,./examples_dir/test_dir/hello_file:1704060920:f,",
            FTree::new(PathBuf::from("./examples_dir"))
                .unwrap()
                .as_string()
                .unwrap()
        );
    }
}
