use std::error::Error;

#[allow(dead_code)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("src/day_2/input.txt")?;

    let points: i32 = file
        .split("\n")
        .into_iter()
        .map(|round| {
            let split = round.split(" ").collect::<Vec<&str>>();
            if split.len() < 2 {
                return 0;
            }

            let enemy = split[0];
            let player = split[1];

            calculate_score(&map_shape(enemy), &map_shape(player))
        })
        .sum();

    dbg!(points);
    Ok(())
}

fn map_shape(code: &str) -> Shape {
    match code {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissor,
        _ => panic!(),
    }
}

fn calculate_score(enemy: &Shape, player: &Shape) -> i32 {
    let points_for_outcome = match enemy {
        Shape::Rock => match player {
            Shape::Rock => 3,
            Shape::Paper => 6,
            Shape::Scissor => 0,
        },
        Shape::Paper => match player {
            Shape::Rock => 0,
            Shape::Paper => 3,
            Shape::Scissor => 6,
        },
        Shape::Scissor => match player {
            Shape::Rock => 6,
            Shape::Paper => 0,
            Shape::Scissor => 3,
        },
    };

    let points_for_shape = match player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };

    points_for_outcome + points_for_shape
}

enum Shape {
    Rock,
    Paper,
    Scissor,
}
