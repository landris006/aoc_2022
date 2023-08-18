use std::{collections::HashMap, error::Error};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_6/input.txt")?;
    let mut chars_checked = 0;
    let mut last_four: Vec<char> = Vec::new();

    for char in input.chars() {
        chars_checked += 1;
        last_four.push(char);

        if last_four.len() < 4 {
            continue;
        }

        if last_four.len() == 5 {
            last_four.remove(0);
        }

        if is_unique(&last_four) {
            break;
        }
    }

    dbg!(chars_checked);

    Ok(())
}

fn is_unique(chars: &[char]) -> bool {
    let mut counter: HashMap<&char, u32> = HashMap::new();

    chars.iter().for_each(|char| {
        counter
            .entry(char)
            .and_modify(|prev| *prev += 1)
            .or_insert(1);
    });

    counter.iter().all(|(_, count)| *count == 1)
}
