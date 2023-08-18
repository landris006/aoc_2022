use std::{cell::RefCell, error::Error};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let crates_input = std::fs::read_to_string("src/day_5/crates.txt")?;
    let instructions_input = std::fs::read_to_string("src/day_5/instructions.txt")?;

    let mut crates = parse_crates(&crates_input);
    let instructions = parse_instructions(&instructions_input);

    instructions.iter().for_each(|instruction| {
        let mut stack_from = crates
            .get((instruction.from - 1) as usize)
            .unwrap()
            .borrow_mut();
        let mut stack_to = crates
            .get((instruction.to - 1) as usize)
            .unwrap()
            .borrow_mut();

        let items: Vec<char> = stack_from
            .iter()
            .rev()
            .take(instruction.repeat as usize)
            .rev()
            .copied()
            .collect();

        items.iter().for_each(|item| {
            stack_from.pop();
        });
        stack_to.extend(items.iter());
    });

    let top_crates: String = crates
        .iter()
        .filter_map(|stack| stack.borrow().last().cloned())
        .collect();
    dbg!(top_crates);

    Ok(())
}

fn parse_crates(input: &str) -> Vec<RefCell<Vec<char>>> {
    let mut crates: Vec<RefCell<Vec<char>>> = vec![RefCell::new(Vec::new()); 9];

    input.split('\n').for_each(|row| {
        (1..row.len()).step_by(4).for_each(|index| {
            let item = row.chars().collect::<Vec<char>>()[index];
            let crate_number = (index - 1) / 4;

            if !item.is_alphabetic() {
                return;
            }

            crates
                .get_mut(crate_number)
                .unwrap()
                .borrow_mut()
                .push(item);
        })
    });
    crates
        .iter_mut()
        .for_each(|stack| stack.borrow_mut().reverse());

    crates
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .filter_map(|row| {
            if row.is_empty() {
                return None;
            }

            let numbers: Vec<u32> = row.split(' ').filter_map(|str| str.parse().ok()).collect();

            if numbers.len() != 3 {
                panic!()
            }

            Some(Instruction {
                repeat: numbers[0],
                from: numbers[1],
                to: numbers[2],
            })
        })
        .collect()
}

struct Instruction {
    repeat: u32,
    from: u32,
    to: u32,
}
