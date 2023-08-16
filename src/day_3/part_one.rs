use std::{char, error::Error};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("src/day_3/input.txt")?;

    let mut duplicates = String::new();

    file.split('\n').for_each(|rucksack| {
        if rucksack.len() < 2 {
            return;
        }

        let first_part = &rucksack[0..(rucksack.len() / 2)];
        let second_part = &rucksack[(rucksack.len() / 2)..];

        let mut duplicates_in_compartment = String::new();
        second_part.chars().for_each(|item| {
            if first_part.contains(item) && !duplicates_in_compartment.contains(item) {
                duplicates_in_compartment.push(item);
            }
        });
        duplicates.push_str(&duplicates_in_compartment);
    });

    dbg!(duplicates
        .chars()
        .map(|char| calculate_priority(&char).unwrap_or(0))
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
