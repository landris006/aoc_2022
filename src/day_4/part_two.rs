use std::error::Error;

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("src/day_4/input.txt")?;

    let mut overlaps = 0;
    file.split('\n')
        .filter(|row| !(*row).eq(""))
        .for_each(|row| {
            let pairs: Vec<&str> = row.split(',').collect();
            let (pair_one, pair_two) = (
                parse_range(pairs[0]).unwrap_or((0, 0)),
                parse_range(pairs[1]).unwrap_or((1, 1)),
            );

            if check_overlap(&pair_one, &pair_two) {
                overlaps += 1;
            }
        });

    dbg!(overlaps);

    Ok(())
}

fn parse_range(str: &str) -> Option<(u8, u8)> {
    let split = str
        .split('-')
        .map(|str| str.parse())
        .collect::<Result<Vec<u8>, _>>()
        .ok()?;

    let (start, end) = (split[0], split[1]);

    Some((start, end))
}

fn check_overlap(pair_one: &(u8, u8), pair_two: &(u8, u8)) -> bool {
    let (min1, max1) = pair_one;
    let (min2, max2) = pair_two;

    if min1 <= min2 && max1 >= min2 {
        return true;
    }

    if max1 >= max2 && min1 <= max2 {
        return true;
    }

    if min1 <= min2 && max1 >= max2 {
        return true;
    }

    if min1 >= min2 && max1 <= max2 {
        return true;
    }

    false
}
