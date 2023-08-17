use std::{error::Error, ops::RangeInclusive};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("src/day_4/input.txt")?;

    let mut number_of_fully_contained = 0;
    file.split('\n')
        .filter(|row| !(*row).eq(""))
        .for_each(|row| {
            let pairs: Vec<&str> = row.split(',').collect();
            let (pair_one, pair_two) = (
                parse_range(pairs[0]).unwrap_or(0..=0),
                parse_range(pairs[1]).unwrap_or(0..=0),
            );

            let only_in_pair_one: Vec<u8> = pair_one
                .clone()
                .filter(|id| !pair_two.contains(id))
                .collect();
            let only_in_pair_two: Vec<u8> = pair_two.filter(|id| !pair_one.contains(id)).collect();

            if only_in_pair_one.is_empty() || only_in_pair_two.is_empty() {
                number_of_fully_contained += 1;
            }
        });

    dbg!(number_of_fully_contained);

    Ok(())
}

fn parse_range(str: &str) -> Option<RangeInclusive<u8>> {
    let split = str
        .split('-')
        .map(|str| str.parse())
        .collect::<Result<Vec<u8>, _>>()
        .ok()?;

    let (start, end) = (split[0], split[1]);

    Some(start..=end)
}
