use std::{char, error::Error};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("src/day_3/input.txt")?;

    let mut duplicates = String::new();

    let mut groups_of_three: Vec<Vec<&str>> = Vec::new();
    let mut current_group: Vec<&str> = Vec::new();

    file.split('\n').for_each(|rucksack| {
        if rucksack.len() < 2 {
            return;
        }

        current_group.push(rucksack);

        if current_group.len() == 3 {
            groups_of_three.push(current_group.clone());
            current_group.clear();
        }
    });

    let mut duplicates = String::new();

    groups_of_three.iter().for_each(|group| {
        for item in group[0].chars() {
            if group[1].contains(item) && group[2].contains(item) {
                duplicates.push(item);
                break;
            }
        }
    });

    dbg!(duplicates
        .chars()
        .map(|letter| calculate_priority(&letter).unwrap_or(0))
        .sum::<u32>());

    Ok(())
}

fn calculate_priority(letter: &char) -> Option<u32> {
    let digit = letter.to_digit(36)?;

    if letter.is_uppercase() {
        Some(digit + 17)
    } else {
        Some(digit - 9)
    }
}
