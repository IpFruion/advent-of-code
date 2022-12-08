use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::Display,
    marker::PhantomData,
    rc::{Rc, Weak},
    str::FromStr,
};

use advent_of_code::errors::Error;

#[derive(Clone)]
pub enum SystemType {
    File(String),
    Directory(String, Vec<FileSystem>),
}

struct FS {
    system_type: SystemType,
    size: Option<usize>,
    parent: Option<Weak<RefCell<FS>>>,
}

#[derive(Clone)]
pub struct FileSystem(Rc<RefCell<FS>>);

macro_rules! write_with_tabs {
    ($dst:expr, $d:ident, $($arg:tt)*) => {
        for _ in 0..$d {
            write!($dst, "\t")?;
        }
        write!($dst, $($arg)*)?;
    }
}

impl FileSystem {
    fn new(fs: FS) -> Self {
        FileSystem(Rc::new(RefCell::new(fs)))
    }

    pub fn new_file<S: ToString>(name: S, size: usize) -> FileSystem {
        Self::new(FS {
            system_type: SystemType::File(name.to_string()),
            size: Some(size),
            parent: None,
        })
    }

    pub fn new_directory<S: ToString>(name: S) -> FileSystem {
        Self::new(FS {
            system_type: SystemType::Directory(name.to_string(), Vec::new()),
            size: None,
            parent: None,
        })
    }

    /// Adds a file system to the children of `self`
    ///
    /// Note: the self is `&mut self` this is to ensure that users of this function must have this
    /// `FileSystem` as a mutable reference. It will work without the mutablility marker but this
    /// actually does change the inner reference. This should be reflected here.
    pub fn add_file_system(&mut self, fs: FileSystem) -> Result<(), Error> {
        let mut from_fs = self.0.try_borrow_mut()?;

        match &mut from_fs.system_type {
            SystemType::File(_) => Err(Error::InvalidStruct(
                "can't add sub file to a file".to_owned(),
            )),
            SystemType::Directory(_, children) => {
                // This has to be scoped so that the borrow gets freed and the to can be pushed.
                {
                    let mut to_fs = fs.0.try_borrow_mut()?;
                    to_fs.parent = Some(Rc::downgrade(&self.0));
                }
                children.push(fs);
                Ok(())
            }
        }
    }

    pub fn parent(&self) -> Result<FileSystem, Error> {
        Ok(FileSystem(
            self.0
                .try_borrow()?
                .parent
                .as_ref()
                .ok_or("No parent on file system")?
                .upgrade()
                .ok_or(Error::BorrowError("unable to get parent".to_string()))?,
        ))
    }

    pub fn find_dir(&self, s: &str) -> Result<Option<FileSystem>, Error> {
        let current = self.0.try_borrow()?;
        match &current.system_type {
            SystemType::File(_) => Err(Error::RawError("No directories in a file".to_owned())),
            SystemType::Directory(_, children) => {
                for c in children.iter() {
                    let child = c.0.try_borrow()?;
                    let found = match &child.system_type {
                        SystemType::File(name) if name == s => true,
                        SystemType::Directory(name, _) if name == s => true,
                        _ => false,
                    };
                    if found {
                        return Ok(Some(FileSystem(c.0.clone())));
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn ref_counts(&self) -> (usize, usize) {
        (Rc::strong_count(&self.0), Rc::weak_count(&self.0))
    }

    pub fn with_depth<'a>(&'a self, depth: usize) -> DepthFileSystem {
        DepthFileSystem(depth, self)
    }

    fn write_with_depth(&self, f: &mut std::fmt::Formatter<'_>, depth: isize) -> std::fmt::Result {
        let mut queue = VecDeque::new();
        queue.push_back((self.clone(), 0));

        while let Some((current, d)) = queue.pop_back() {
            let current = current.0.borrow();
            match &current.system_type {
                SystemType::File(name) => {
                    write_with_tabs!(f, d, "File: {}", name);
                }
                SystemType::Directory(name, children) => {
                    write_with_tabs!(f, d, "Directory {}", name);
                    if depth < 0 || d + 1 <= depth {
                        for child in children.iter() {
                            queue.push_back((child.clone(), d + 1))
                        }
                    }
                }
            };
            writeln!(f, " Size: {:?}", current.size)?;
        }

        Ok(())
    }

    pub fn update_sizes(&mut self) -> Result<(), Error> {
        self.update_size_rec()?;
        Ok(())
        // TODO: Figure out how to do non recursive strategy
        // let mut queue = VecDeque::new();
        // queue.push_back(self.clone());
        //
        //
        // while let Some(current) = queue.pop_back() {
        //     let cur = current.0.try_borrow()?;
        //     if let (Some(parent), Some(cur_size)) = (&cur.parent, cur.size) {
        //         let parent = parent
        //             .upgrade()
        //             .ok_or(Error::BorrowError("can't upgrade parent".to_owned()))?;
        //         let mut parent = parent.try_borrow_mut()?;
        //
        //         if let Some(size) = parent.size.as_mut() {
        //             *size += cur_size
        //         } else {
        //             parent.size = Some(cur_size)
        //         }
        //     }
        //     match &cur.system_type {
        //         SystemType::File(_) => {}
        //         SystemType::Directory(_, children) => {
        //             for c in children.iter() {
        //                 queue.push_back(c.clone())
        //             }
        //         }
        //     }
        // }
    }

    fn update_size_rec(&mut self) -> Result<usize, Error> {
        let mut fs = self.0.try_borrow_mut()?;
        match &mut fs.system_type {
            SystemType::File(_) => fs.size.ok_or("No Size for File Invalid".into()),
            SystemType::Directory(_, children) => {
                let mut total = 0;
                for c in children.iter_mut() {
                    total += c.update_size_rec()?;
                }
                fs.size = Some(total);
                Ok(total)
            }
        }
    }

    pub fn iter<'a>(&'a self, iter_type: FSIterType) -> FSIterator<'a> {
        FSIterator {
            iter_type,
            queue: VecDeque::from([self.clone()]),
            _queue_phantom: Default::default(),
        }
    }

    pub fn size(&self) -> Result<usize, Error> {
        self.0
            .try_borrow()
            .map(|s| s.size.unwrap_or(0))
            .map_err(|e| e.into())
    }

    pub fn system_type(&self) -> Result<SystemType, Error> {
        let sys_type = self.0.try_borrow()?;
        Ok(sys_type.system_type.clone())
    }
}

pub struct DepthFileSystem<'a>(usize, &'a FileSystem);

impl<'a> Display for DepthFileSystem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.1.write_with_depth(f, self.0 as isize)
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_with_depth(f, -1)
    }
}

impl FromStr for FileSystem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        let file_system_too_short = || Error::InvalidParseError("file system too short".to_owned());
        let indicator = splits.next().ok_or_else(file_system_too_short)?;

        match indicator {
            "dir" => {
                let name = splits.next().ok_or_else(file_system_too_short)?;
                Ok(Self::new_directory(name.to_owned()))
            }
            size => {
                let size = usize::from_str(size)?;
                let name = splits.next().ok_or_else(file_system_too_short)?;
                Ok(Self::new_file(name.to_owned(), size))
            }
        }
    }
}

pub enum FSIterType {
    DepthFirst,
    BreadthFirst,
}

pub struct FSIterator<'a> {
    iter_type: FSIterType,
    queue: VecDeque<FileSystem>,
    _queue_phantom: PhantomData<&'a FileSystem>,
}

impl<'a> Iterator for FSIterator<'a> {
    //TODO: figure out how to make it &'a FileSystem
    type Item = FileSystem;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.iter_type {
            FSIterType::DepthFirst => self.queue.pop_back(),
            FSIterType::BreadthFirst => self.queue.pop_front(),
        }?;
        match &next.0.try_borrow().ok()?.system_type {
            SystemType::File(_) => {}
            SystemType::Directory(_, children) => {
                for c in children.iter() {
                    self.queue.push_back(c.clone())
                }
            }
        };

        Some(next.clone())
    }
}
