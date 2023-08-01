use std::{io, collections::HashMap, rc::Rc, cell::RefCell};

struct Dir {
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdirs: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn size(&self) -> usize {
        *self.size.borrow() + self.subdirs.borrow().values().map(|dir| dir.size()).sum::<usize>()
    }
}

struct Tree {
    root: Rc<Dir>,
}

impl Tree {
    fn solve1(&self) -> usize {
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut result = 0;

        while let Some(dir) = to_visit.pop() {
            for d in dir.subdirs.borrow().values() {
                to_visit.push(Rc::clone(&d.clone()));
            }
            let size = dir.size();
            if size <= 100_000 {
                result += size;
            }
        }
        result
    }

    fn solve2(&self) -> usize {
        let used_space = self.root.size();
        let free_space = 70_000_000 - used_space;
        let enough_space = 30_000_000 - free_space;

        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut min: usize = usize::MAX;

        while let Some(dir) = to_visit.pop() {
            for d in dir.subdirs.borrow().values() {
                to_visit.push(Rc::clone(&d.clone()));
            }
            let size = dir.size();
            if size >= enough_space {
                min = min.min(size);
            }
        }
        min
    }
}

fn load_stdin() -> Result<String, io::Error> {
    let mut input: String = String::new();
    loop {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        input.push_str(&line);
    }
    Ok(input)
}

fn main() -> io::Result<()> {
    let input = load_stdin()?;
    let dir_tree: Tree = Tree {
        root: Rc::new(Dir {
            size: RefCell::new(0),
            parent: None,
            subdirs: RefCell::new(HashMap::new()),
        }),
    };
    let mut cwd = Rc::clone(&dir_tree.root);

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&dir_tree.root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dir_name => {
                    let dir = cwd.subdirs.borrow().get(dir_name).unwrap().clone();
                    cwd = dir;
                }
            }
            ("$", "ls") => {},
            ("dir", dir_name) => {
                let new_dir = Rc::new(Dir {
                    size: RefCell::new(0),
                    parent: Some(Rc::clone(&cwd)),
                    subdirs: RefCell::new(HashMap::new()),
                });
                cwd.subdirs.borrow_mut().insert(dir_name.to_string(), Rc::clone(&new_dir));
            }
            (size, _) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    println!("The result is {}", dir_tree.solve1());
    println!("The result is {}", dir_tree.solve2());
    Ok(())
}