use std::error::Error;

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_8/input.txt")?;

    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|tree| tree.to_digit(10)).collect())
        .collect();

    let highest_score = forest
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, height)| calculate_score(height, &forest, (j, i)))
                .collect::<Vec<u32>>()
        })
        .max()
        .unwrap_or(0);

    dbg!(highest_score);

    Ok(())
}

fn calculate_score(
    current_height: &u32,
    forest: &Vec<Vec<u32>>,
    coordinates: (usize, usize),
) -> u32 {
    let (x, y) = coordinates;
    let forest_width = forest.get(0).unwrap().len();
    let forest_height = forest.len();

    let mut score_left = 0;
    for x in (0..x).rev() {
        score_left += 1;
        let height = forest.get(y).unwrap().get(x).unwrap();
        if height >= current_height {
            break;
        };
    }

    let mut score_right = 0;
    for x in (x + 1)..forest_width {
        score_right += 1;
        let height = forest.get(y).unwrap().get(x).unwrap();
        if height >= current_height {
            break;
        };
    }

    let mut score_top = 0;
    for y in (0..y).rev() {
        score_top += 1;
        let height = forest.get(y).unwrap().get(x).unwrap();
        if height >= current_height {
            break;
        };
    }

    let mut score_bottom = 0;
    for y in (y + 1)..forest_height {
        score_bottom += 1;
        let height = forest.get(y).unwrap().get(x).unwrap();
        if height >= current_height {
            break;
        };
    }

    score_left * score_right * score_top * score_bottom
}
