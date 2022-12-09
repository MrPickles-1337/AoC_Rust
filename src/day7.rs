#[derive(Debug, Clone)]
pub struct File(usize);

#[derive(Debug, Clone)]
pub struct Dir {
    path: String,
    size: usize,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

impl Dir {
    fn new(path: &str) -> Dir {
        return Dir {
            path: String::from(path),
            size: 0,
            dirs: Vec::new(),
            files: Vec::new(),
        };
    }

    fn calc_self_size(&mut self) -> usize {
        self.size = self.files.iter().map(|f| f.0).sum();
        if self.dirs.is_empty() {
            return self.size;
        }
        let sub_dirs_size: usize = self.dirs.iter_mut().map(|d| d.calc_self_size()).sum();
        self.size += sub_dirs_size;

        return self.size;
    }

    fn get_dirs_up_to_100k(&self) -> Vec<&Dir> {
        let mut result = Vec::new();
        if self.size <= 100_000 {
            result.push(self);
        }
        if !self.dirs.is_empty() {
            self.dirs
                .iter()
                .for_each(|d| result.append(&mut d.get_dirs_up_to_100k()));
        }
        return result;
    }

    fn collect(&self) -> Vec<&Dir> {
        let mut result = Vec::new();
        result.push(self);
        if !self.dirs.is_empty() {
            self.dirs
                .iter()
                .for_each(|d| result.append(&mut d.collect()));
        }

        return result;
    }

    fn get_by_path(&mut self, path: &str) -> &mut Dir {
        let mut dir_names = path.split("/").collect::<Vec<&str>>();
        dir_names.remove(0);
        let first_path = dir_names.remove(0);
        if path == "" || path == "/" {
            return self;
        }
        if dir_names.len() == 0 {
            return self
                .dirs
                .iter_mut()
                .find(|d| d.path.as_str() == format!("/{}", first_path))
                .unwrap();
        } else {
            return self
                .dirs
                .iter_mut()
                .find(|d| d.path.as_str() == format!("/{}", first_path))
                .unwrap()
                .get_by_path(&format!("/{}", dir_names.join("/").as_str()));
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Dir {
    let mut root = Dir::new("/");
    let mut current_path = String::from("");
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        if line.starts_with('$') {
            let spl = line.split(" ").collect::<Vec<&str>>();
            if spl.len() == 3 {
                if spl.last().unwrap() == &".." {
                    let mut spl_paths = current_path.split("/").collect::<Vec<&str>>();
                    spl_paths.pop();
                    current_path = spl_paths.join("/");
                    if current_path.is_empty() {
                        current_path = String::from("/");
                    }
                } else {
                    if current_path.as_str() == "/" || current_path.is_empty() {
                        current_path.push_str(spl.last().unwrap());
                    } else {
                        current_path.push_str(format!("/{}", spl.last().unwrap()).as_str());
                    }
                }
                continue;
            }
        } else {
            let spl = line.split(" ").collect::<Vec<&str>>();
            if let Ok(size) = spl.first().unwrap().parse::<usize>() {
                root.get_by_path(&current_path).files.push(File(size));
            } else {
                let path = format!("/{}", spl.last().unwrap());
                root.get_by_path(&current_path).dirs.push(Dir::new(&path));
            }
        }
    }
    return root;
}

#[aoc(day7, part1)]
pub fn part1(input: &Dir) -> usize {
    let mut input = input.clone();
    input.calc_self_size();
    input.get_dirs_up_to_100k().iter().map(|d| d.size).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Dir) -> usize {
    let mut input = input.clone();
    input.calc_self_size();
    let free_size = 70_000_000 - input.size;
    let need_to_free = 30_000_000 - free_size;
    let mut res = 70_000_000;
    for dir in input.collect() {
        if dir.size >= need_to_free && dir.size <= res {
            res = dir.size;
        }
    }
    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "$ cd /
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
7214296 k";

        assert_eq!(95437, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "$ cd /
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
7214296 k";

        assert_eq!(24933642, part2(&input_generator(input)));
    }
}
