use alloc::collections::btree_map::BTreeMap;
use spin::RwLock;

#[derive(Clone)]
enum Node {
    File(Vec<u8>),
    Directory(BTreeMap<Arc<str>, Node>),
}

pub struct InMemoryFileSystem {
    root: Arc<RwLock<FsNode>>,
}

impl InMemoryFileSystem {
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(FsNode::Directory(BTreeMap::new()))),
        }
    }

    fn traverse<'a>(
        mut path: impl Iterator<Item = &'a str>,
        node: &'a mut FsNode,
    ) -> Option<&'a mut FsNode> {
        match path.next() {
            Some(part) => match node {
                FsNode::Directory(entries) => {
                    entries.get_mut(part).and_then(|n| Self::traverse(path, n))
                }
                _ => None,
            },
            None => Some(node),
        }
    }

    fn traverse_mut(&self, path: &Path) -> Option<std::sync::RwLockWriteGuard<FsNode>> {
        let mut guard = self.root.write().ok()?;
        let mut node = &mut *guard;

        for part in path.iter().map(|s| s.to_str().unwrap_or("")) {
            if let FsNode::Directory(entries) = node {
                node = entries.get_mut(part)?;
            } else {
                return None;
            }
        }

        Some(guard)
    }

    fn create_dirs_recursive(&self, path: &Path) -> Result<(), u32> {
        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;

        for component in path.iter().map(|s| s.to_str().unwrap_or("")) {
            match current {
                FsNode::Directory(ref mut children) => {
                    current = children
                        .entry(component.to_string())
                        .or_insert_with(|| FsNode::Directory(HashMap::new()));
                }
                _ => return Err(2),
            }
        }

        Ok(())
    }

    fn get_node(&self, path: &Path) -> Option<FsNode> {
        let mut node = self.root.read().ok()?.clone();
        for component in path.iter().map(|s| s.to_str().unwrap_or("")) {
            if let FsNode::Directory(entries) = node {
                node = entries.get(component)?.clone();
            } else {
                return None;
            }
        }
        Some(node)
    }
}

impl FileSystem for InMemoryFileSystem {
    fn read_file(&self, path: Path) -> Result<Vec<u8>, u32> {
        match self.get_node(&path) {
            Some(FsNode::File(data)) => Ok(data),
            _ => Err(1),
        }
    }

    fn mkdir(&self, path: Path) -> Result<(), u32> {
        let parent = path.parent().unwrap_or(Path::new(""));
        self.create_dirs_recursive(parent)?;

        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;
        for component in path.iter().map(|s| s.to_str().unwrap_or("")) {
            match current {
                FsNode::Directory(children) => {
                    current = children
                        .entry(component.to_string())
                        .or_insert_with(|| FsNode::Directory(HashMap::new()));
                }
                _ => return Err(2),
            }
        }

        Ok(())
    }

    fn mkdirs(&self, path: Path) -> Result<(), u32> {
        self.create_dirs_recursive(&path)
    }

    fn remove_dir_contents(&self, path: Path) -> Result<(), u32> {
        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;

        for component in path.iter().map(|s| s.to_str().unwrap_or("")) {
            match current {
                FsNode::Directory(children) => {
                    current = children.get_mut(component).ok_or(2)?;
                }
                _ => return Err(3),
            }
        }

        if let FsNode::Directory(children) = current {
            children.clear();
            Ok(())
        } else {
            Err(4)
        }
    }

    fn remove_dir(&self, path: Path) -> Result<(), u32> {
        let parent = path.parent().unwrap_or(Path::new(""));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or(1)?;

        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;
        for component in parent.iter().map(|s| s.to_str().unwrap_or("")) {
            current = match current {
                FsNode::Directory(children) => children.get_mut(component).ok_or(2)?,
                _ => return Err(3),
            };
        }

        if let FsNode::Directory(children) = current {
            if let Some(FsNode::Directory(dir)) = children.get(name) {
                if dir.is_empty() {
                    children.remove(name);
                    Ok(())
                } else {
                    Err(4)
                }
            } else {
                Err(5)
            }
        } else {
            Err(6)
        }
    }

    fn remove_dir_all(&self, path: Path) -> Result<(), u32> {
        let parent = path.parent().unwrap_or(Path::new(""));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or(1)?;

        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;

        for component in parent.iter().map(|s| s.to_str().unwrap_or("")) {
            current = match current {
                FsNode::Directory(children) => children.get_mut(component).ok_or(2)?,
                _ => return Err(3),
            };
        }

        if let FsNode::Directory(children) = current {
            children.remove(name);
            Ok(())
        } else {
            Err(4)
        }
    }

    fn remove_file(&self, path: Path) -> Result<(), u32> {
        let parent = path.parent().unwrap_or(Path::new(""));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or(1)?;

        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;

        for component in parent.iter().map(|s| s.to_str().unwrap_or("")) {
            current = match current {
                FsNode::Directory(children) => children.get_mut(component).ok_or(2)?,
                _ => return Err(3),
            };
        }

        if let FsNode::Directory(children) = current {
            if let Some(FsNode::File(_)) = children.get(name) {
                children.remove(name);
                Ok(())
            } else {
                Err(4)
            }
        } else {
            Err(5)
        }
    }

    fn create_file(&self, path: Path) -> Result<(), u32> {
        let parent = path.parent().unwrap_or(Path::new(""));
        self.create_dirs_recursive(parent)?;

        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;

        for component in path
            .parent()
            .unwrap()
            .iter()
            .map(|s| s.to_str().unwrap_or(""))
        {
            current = match current {
                FsNode::Directory(children) => children.get_mut(component).ok_or(2)?,
                _ => return Err(3),
            };
        }

        if let FsNode::Directory(children) = current {
            let filename = path.file_name().and_then(|s| s.to_str()).ok_or(4)?;
            children.insert(filename.to_string(), FsNode::File(vec![]));
            Ok(())
        } else {
            Err(5)
        }
    }

    fn write_file(&self, path: Path, data: &[u8]) -> Result<(), u32> {
        self.create_file(path.clone())?;
        let mut guard = self.root.write().map_err(|_| 1)?;
        let mut current = &mut *guard;

        for component in path.iter().map(|s| s.to_str().unwrap_or("")) {
            match current {
                FsNode::Directory(children) => {
                    current = children.get_mut(component).ok_or(2)?;
                }
                _ => return Err(3),
            }
        }

        if let FsNode::File(ref mut file) = current {
            *file = data.to_vec();
            Ok(())
        } else {
            Err(4)
        }
    }

    fn list_files_filtered<F>(&self, path: Path, filter: &F) -> Option<Vec<Path>>
    where
        F: Fn(&Path) -> bool,
    {
        let mut results = vec![];
        let mut node = self.get_node(&path)?;

        if let FsNode::Directory(children) = &mut node {
            for (name, _) in children.iter() {
                let child_path = path.join(name);
                if filter(&child_path) {
                    results.push(child_path);
                }
            }
        }

        Some(results)
    }

    fn get_filetime(&self, _path: Path) -> Option<(u32, u32)> {
        Some((0, 0)) // Stubbed, could be extended with timestamps
    }

    fn is_exists(&self, path: Path) -> bool {
        self.get_node(&path).is_some()
    }

    fn is_dir(&self, path: Path) -> bool {
        matches!(self.get_node(&path), Some(FsNode::Directory(_)))
    }

    fn is_file(&self, path: Path) -> bool {
        matches!(self.get_node(&path), Some(FsNode::File(_)))
    }
}
