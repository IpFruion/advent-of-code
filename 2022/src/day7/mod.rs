use self::{
    system::{FSIterType, FileSystem, SystemType},
    term::Command,
};
use advent_of_code::errors::Result;

mod system;
mod term;

fn build_file_system<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<FileSystem> {
    let mut root = FileSystem::new_directory("/");
    let mut current = root.clone();

    for line in lines {
        let line = line.as_ref().trim();
        if line.is_empty() {
            continue;
        }
        let cmd: Result<Command> = line.parse();
        let output: Result<FileSystem> = line.parse();
        match (cmd, output) {
            (Err(_), Ok(line)) => current.add_file_system(line)?,
            (Ok(cmd), Err(_)) => match cmd {
                Command::Cd(name) => {
                    current = match name.as_ref() {
                        ".." => current.parent()?,
                        "/" => root.clone(),
                        s => current.find_dir(s)?.ok_or("No such directory found")?,
                    }
                }
                // do nothing because it is going to start reading
                Command::Ls => {}
            },
            (Err(e), Err(_)) => return Err(e),
            _ => Err("Invalid Input")?,
        };
    }
    root.update_sizes()?;
    Ok(root)
}

pub fn solution_pt1<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let fs = build_file_system(lines)?;
    println!("File System: \n{}", fs);

    fs.iter(FSIterType::BreadthFirst)
        .map(|v| {
            let size = v.size()?;
            if size <= 100_000 && matches!(v.system_type()?, SystemType::Directory(_, _)) {
                Ok(size)
            } else {
                Ok(0)
            }
        })
        .sum()
}

pub fn solution_pt2<S: AsRef<str>, L: Iterator<Item = S>>(lines: L) -> Result<usize> {
    let fs = build_file_system(lines)?;
    println!("File System: \n{}", fs);

    let unused = 70_000_000 - fs.size()?;
    let need_space = 30_000_000 - unused;

    fs.iter(FSIterType::BreadthFirst)
        .filter(|v| {
            v.system_type()
                .map(|s| matches!(s, SystemType::Directory(_, _)))
                .unwrap_or(false)
        })
        .fold(Ok(None), |accum, v| {
            let min = accum?;
            let size = v.size()?;
            if size >= need_space {
                println!("{}", v.with_depth(0));
            }
            Ok(match min {
                None => Some(size),
                Some(min) if size >= need_space && size < min => Some(size),
                Some(min) => Some(min),
            })
        })
        .and_then(|min| min.ok_or("Minimum not found".into()))
}

#[cfg(test)]
mod tests {
    use super::{solution_pt1, solution_pt2};

    const PAGE_EXAMPLE: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;
    #[test]
    fn page_example_1() {
        let mut lines = PAGE_EXAMPLE.split("\n");
        lines.next().unwrap();

        let actual = solution_pt1(lines).unwrap();

        assert_eq!(actual, 95437)
    }

    #[test]
    fn page_example_2() {
        let mut lines = PAGE_EXAMPLE.split("\n");
        lines.next().unwrap();

        let actual = solution_pt2(lines).unwrap();

        assert_eq!(actual, 24933642)
    }
}
