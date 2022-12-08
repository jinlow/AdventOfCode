use std::collections::HashMap;
use std::error::Error;
use std::{fmt, fs};

fn is_command(s: &str) -> bool {
    s.starts_with("$")
}

fn cd_command(s: &str) -> Option<&str> {
    if s.starts_with("$ cd") {
        s.split(" cd ").nth(1)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    sub_dirs: HashMap<String, usize>,
    parent_dir: Option<usize>,
    files: Vec<File>,
    total_size: usize,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (dir)", self.name)
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (file, size={})", self.name, self.size)
    }
}

struct FileSystem {
    dirs: Vec<Dir>,
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut r = String::new();
        // Dir index, and recursion depth...
        let mut print_buffer: Vec<(usize, usize)> = vec![(0, 0)];
        while !print_buffer.is_empty() {
            let idx = print_buffer.pop().unwrap();
            let n = &self.dirs[idx.0];
            r += format!("{}- {}\n", " ".repeat(idx.1), n).as_str();
            // add all files, and directories, to the print_buffer
            for f in n.files.iter() {
                r += format!("{}- {}\n", " ".repeat(idx.1 + 1), f).as_str();
            }
            for dir in self.dirs[idx.0].sub_dirs.values() {
                print_buffer.push((*dir, idx.1 + 1));
            }
        }
        write!(f, "{}", r)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;

    let mut fsys = FileSystem { dirs: Vec::new() };
    fsys.dirs.push(Dir {
        name: String::from("/"),
        sub_dirs: HashMap::new(),
        parent_dir: None,
        files: Vec::new(),
        total_size: 0,
    });
    let mut cwd = 0;
    for l in file.lines() {
        if is_command(l) {
            match cd_command(l) {
                // We have CD'd into a new working directory.
                Some(d) => match d {
                    "/" => cwd = 0,
                    ".." => cwd = fsys.dirs[cwd].parent_dir.unwrap(),
                    _ => match fsys.dirs[cwd].sub_dirs.get(d) {
                        None => {
                            let new_dir = Dir {
                                name: d.to_string(),
                                sub_dirs: HashMap::new(),
                                parent_dir: Some(cwd),
                                files: Vec::new(),

                                total_size: 0,
                            };
                            let new_idx = fsys.dirs.len();
                            fsys.dirs[cwd].sub_dirs.insert(d.to_string(), new_idx);
                            cwd = new_idx;
                            fsys.dirs.push(new_dir);
                        }
                        Some(ed) => cwd = *ed,
                    },
                },
                // Otherwise it's an ls command.
                // Which we don't actually need to do anything for,
                // The cd is all that matters, and then if it's not a
                // command, it's going to be the CWD content.
                None => (),
            }
        } else {
            if l.starts_with("dir") {
                let dir_name = l.split(" ").nth(1).unwrap().to_string();
                let new_dir = Dir {
                    name: dir_name.to_string(),
                    sub_dirs: HashMap::new(),
                    parent_dir: Some(cwd),
                    files: Vec::new(),

                    total_size: 0,
                };
                let new_idx = fsys.dirs.len();
                fsys.dirs[cwd]
                    .sub_dirs
                    .insert(dir_name.to_string(), new_idx);
                fsys.dirs.push(new_dir);
            } else {
                let file_info = l.split(" ").collect::<Vec<&str>>();
                let new_file = File {
                    name: file_info[1].to_string(),
                    size: file_info[0].parse::<usize>()?,
                };
                fsys.dirs[cwd].total_size += new_file.size;
                // Climb back up the parents of this directory, and add the file size.
                let mut parent_buffer = Vec::new();
                if let Some(x) = fsys.dirs[cwd].parent_dir {
                    parent_buffer.push(x);
                    while !parent_buffer.is_empty() {
                        let p = parent_buffer.pop().unwrap();
                        fsys.dirs[p].total_size += new_file.size;
                        if let Some(xp) = fsys.dirs[p].parent_dir {
                            parent_buffer.push(xp);
                        }
                    }
                }

                fsys.dirs[cwd].files.push(new_file);
            }
        }
    }

    //println!("{}", fsys);
    //println!("{}, {}", total_size, fsys.dirs[0].total_size);
    // Calculate all directory sizes...
    // P1
    let mut sizes = Vec::new();
    let mut sizes_names = Vec::new();
    let mut size_buffer = vec![0];
    while !size_buffer.is_empty() {
        let p = size_buffer.pop().unwrap();
        sizes.push(fsys.dirs[p].total_size);
        sizes_names.push(fsys.dirs[p].name.clone());
        let sub_dirs = fsys.dirs[p]
            .sub_dirs
            .values()
            .map(|x| *x)
            .collect::<Vec<usize>>();
        size_buffer.extend(sub_dirs);
    }
    let l1k = sizes.iter().filter(|x| x < &&100000).sum::<usize>();
    println!("{}", l1k);

    // P2
    let free_size = 30000000 - (70000000 - fsys.dirs[0].total_size);
    println!("{}", free_size);
    let mut delete_dirs = sizes
        .iter()
        .zip(sizes_names)
        .filter(|(x, _)| x >= &&free_size)
        .map(|(x, name)| (*x, name))
        .collect::<Vec<(usize, String)>>();
    delete_dirs.sort_by(|a, b| a.0.cmp(&b.0));
    //delete_dirs.reverse();
    println!("{:?}", delete_dirs[0..3].to_vec());
    Ok(())
}
