use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

const TOTAL_SPACE: u32 = 70_000_000;
const SPACE_NEEDED: u32 = 30_000_000;

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_7/input.txt")?;

    let root = Rc::new(RefCell::new(File {
        size: None,
        children: HashMap::new(),
    }));

    let mut cwd: Vec<&str> = vec![];
    let mut files_flat: Vec<Rc<RefCell<File>>> = Vec::new();

    input.split('\n').skip(1).for_each(|line| {
        if let Some(first_char) = line.chars().next() {
            let args: Vec<&str> = line.split(' ').collect();

            match first_char {
                '$' => process_command(&mut cwd, &args),
                _ => {
                    if let Some(new_file) = process_file(Rc::clone(&root), &cwd, &args) {
                        files_flat.push(new_file);
                    }
                }
            }
        }
    });

    let allocated_size = root.borrow().compute_size();
    let space_to_free = SPACE_NEEDED + allocated_size - TOTAL_SPACE;

    let mut dirs_large_enough = files_flat
        .into_iter()
        .filter(|file| file.borrow().size.is_none())
        .filter(|dir| dir.borrow().compute_size() >= space_to_free)
        .collect::<Vec<Rc<RefCell<File>>>>();

    dirs_large_enough.sort_by(|a, b| {
        a.borrow()
            .compute_size()
            .partial_cmp(&b.borrow().compute_size())
            .unwrap()
    });
    dbg!(dirs_large_enough.get(0).unwrap().borrow().compute_size());

    Ok(())
}

fn process_command<'a>(cwd: &mut Vec<&'a str>, args: &[&'a str]) {
    if *args.get(1).unwrap() == "cd" {
        match *args.get(2).unwrap() {
            ".." => {
                cwd.pop();
            }
            dir_name => {
                cwd.push(dir_name);
            }
        };
    };
}

fn process_file<'a>(
    root: Rc<RefCell<File>>,
    cwd: &[&'a str],
    args: &[&'a str],
) -> Option<Rc<RefCell<File>>> {
    let file_name = *args.get(1).unwrap();

    let mut file = Rc::clone(&root);
    cwd.iter().for_each(|segment| {
        file = {
            let x = Rc::clone(file.borrow().lookup_child(segment).unwrap());
            x
        };
    });

    if file
        .borrow()
        .children
        .iter()
        .any(|(key, _)| key[..] == *file_name)
    {
        return None;
    }

    match *args.first().unwrap() {
        "dir" => {
            let new_file = Rc::new(RefCell::new(File {
                size: None,
                children: HashMap::new(),
            }));
            file.borrow_mut()
                .children
                .insert(file_name.to_string(), Rc::clone(&new_file));
            Some(new_file)
        }
        file_size => {
            let new_file = Rc::new(RefCell::new(File {
                size: file_size.parse().ok(),
                children: HashMap::new(),
            }));
            file.borrow_mut()
                .children
                .insert(file_name.to_string(), Rc::clone(&new_file));
            Some(new_file)
        }
    }
}

#[derive(Debug)]
struct File {
    size: Option<u32>,
    children: HashMap<String, Rc<RefCell<File>>>,
}

impl File {
    pub fn lookup_child(&self, file_name: &str) -> Option<&Rc<RefCell<File>>> {
        self.children.get(&file_name.to_string())
    }

    pub fn compute_size(&self) -> u32 {
        match self.size {
            Some(number) => number,
            None => {
                let sum: u32 = self
                    .children
                    .values()
                    .map(|file| file.borrow().compute_size())
                    .sum();
                sum
            }
        }
    }
}
