use std::error::Error;

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_8/input.txt")?;

    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|tree| tree.to_digit(10)).collect())
        .collect();

    let nbr_of_visible: usize = forest
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(j, height)| is_visible(height, &forest, (*j, i)))
                .count()
        })
        .sum();

    dbg!(nbr_of_visible);

    Ok(())
}

fn is_visible(current_height: &u32, forest: &Vec<Vec<u32>>, coordinates: (usize, usize)) -> bool {
    let (x, y) = coordinates;
    let forest_width = forest.get(0).unwrap().len();
    let forest_height = forest.len();

    if x == 0 || x == forest_width - 1 {
        return true;
    }
    if y == 0 || y == forest_height - 1 {
        return true;
    }

    let is_visible_from_left = (0..x)
        .map(|x| forest.get(y).unwrap().get(x).unwrap())
        .all(|height| height < current_height);
    if is_visible_from_left {
        return true;
    }

    let is_visible_from_right = ((x + 1)..forest_width)
        .map(|x| forest.get(y).unwrap().get(x).unwrap())
        .all(|height| height < current_height);
    if is_visible_from_right {
        return true;
    }

    let is_visible_from_top = (0..y)
        .map(|y| forest.get(y).unwrap().get(x).unwrap())
        .all(|height| height < current_height);
    if is_visible_from_top {
        return true;
    }

    let is_visible_from_bottom = ((y + 1)..forest_height)
        .map(|y| forest.get(y).unwrap().get(x).unwrap())
        .all(|height| height < current_height);
    if is_visible_from_bottom {
        return true;
    }

    false
}
