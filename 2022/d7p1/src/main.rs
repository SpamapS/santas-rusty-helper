/*
--- Day 7: No Space Left On Device ---

You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device

Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

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

The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

    cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
        cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
        cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
        cd / switches the current directory to the outermost directory, /.
    ls means list. It prints out all of the files and directories immediately contained by the current directory:
        123 abc means that the current directory contains a file named abc with size 123.
        dir xyz means that the current directory contains a directory named xyz.

Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

    The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
    The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
    Directory d has total size 24933642.
    As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.

To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

--- Part Two ---

Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

    Delete directory e, which would increase unused space by 584.
    Delete directory a, which would increase unused space by 94853.
    Delete directory d, which would increase unused space by 24933642.
    Delete directory /, which would increase unused space by 48381165.

Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?


*/

use std::{fs, path::{PathBuf, Path}, collections::{HashMap, BinaryHeap}, cmp::Reverse};

const CD: &'static str = "$ cd ";
const LS: &'static str = "$ ls";


fn parse_cmds<'a> (buf: &'a str, cwd: &'a mut PathBuf) -> Vec<(String, &'a str, usize)> {
    let mut files: Vec<(String, &'a str, usize)>  = Vec::new();
    for line in buf.lines() {
        if line.starts_with(CD) {
            let (_cmd, target) = line.rsplit_once(' ').expect("no space in cd cmd");
            if target == ".." {
                cwd.pop();
            } else {
                cwd.push(target);
            }
            continue;
        }
        if line.starts_with(LS) || line.starts_with("dir") {
            continue;
        }
        // Assuming all else is files
        let (size, path) = line.split_once(' ').expect("No space found in line");
        let size: usize = size.parse().expect(&format!("[{}] size({}) did not parse as number", line, size));
        files.push((String::from(cwd.as_path().clone().to_str().unwrap()), path, size));
    }
    files
}

fn sum_one_dir<'a>(sums: &mut HashMap<&'a str, usize>, path: &'a Path, size: usize) {
    sums.entry(path.to_str().unwrap()).and_modify(|s: &mut usize| *s += size).or_insert(size);
    match path.parent() {
        None => return,
        Some(path) => sum_one_dir(sums, path, size),
    }
}

fn sum_dirs<'a>(input: &'a Vec<(String, &'a str, usize)>) -> HashMap<&'a str, usize> {
    let mut sums = HashMap::new();
    for (dir, _file, size) in input.iter() {
        let p = Path::new(dir);
        sum_one_dir(&mut sums, p, *size);
    }
    sums
}

#[test]
fn test_parse_cmds() {
    let test_input = "$ cd /
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
    let root = String::from("/");
    let a = String::from("/a");
    let ae = String::from("/a/e");
    let d = String::from("/d");

    let test_findings: Vec<(String, &str, usize)> = vec![
        (root.clone(), "b.txt", 14848514),
        (root, "c.dat", 8504156),
        (a.clone(), "f", 29116),
        (a.clone(), "g", 2557),
        (a, "h.lst", 62596),
        (ae, "i", 584),
        (d.clone(), "j", 4060174),
        (d.clone(), "d.log", 8033020),
        (d.clone(), "d.ext", 5626152),
        (d, "k", 7214296),
    ];
    let mut cwd = PathBuf::from("/");
    let parsed_cmds = parse_cmds(test_input, &mut cwd);
    assert_eq!(test_findings, parsed_cmds);
    let sums = sum_dirs(&parsed_cmds);
    assert_eq!(48381165, *sums.get("/").unwrap());
}

const TOTAL_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

fn main() {
    let buf = fs::read_to_string("2022d7p1.txt").unwrap();
    let mut cwd = PathBuf::from("/");
    let parsed_cmds = parse_cmds(&buf, &mut cwd);
    let summed_dirs = sum_dirs(&parsed_cmds);
    summed_dirs.iter().filter(|(_dir, size)| **size <= 100000).for_each(|(dir, size)| println!("Remove {} which has size {}", dir, size));
    println!("The total is: {}", summed_dirs.iter().filter(|(dir, size)| **size <= 100000 && **dir != "/").fold(0, |t, (_dir, size)| t + size));
    let total_used = summed_dirs.get("/").unwrap();
    let unused_space = TOTAL_SPACE - total_used;
    let at_least = NEEDED_SPACE - unused_space;
    let mut suitable_dirs: BinaryHeap<Reverse<usize>> = summed_dirs.iter().map(|(_dir, size)| *size).filter(|size| *size > at_least).map(Reverse).collect();
    let target_size: Reverse<usize> = suitable_dirs.pop().unwrap();
    println!("Removing {:?} bytes", target_size);
}
