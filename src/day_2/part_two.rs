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

            calculate_score(&map_shape(enemy), &map_outcome(player))
        })
        .sum();

    dbg!(points);
    Ok(())
}

fn map_shape(code: &str) -> Shape {
    match code {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        _ => panic!(),
    }
}
fn map_outcome(code: &str) -> Outcome {
    match code {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!(),
    }
}

fn calculate_score(enemy: &Shape, outcome: &Outcome) -> i32 {
    match enemy {
        Shape::Rock => match outcome {
            Outcome::Win => 6 + 2,
            Outcome::Draw => 3 + 1,
            Outcome::Lose => 0 + 3,
        },
        Shape::Paper => match outcome {
            Outcome::Win => 6 + 3,
            Outcome::Draw => 3 + 2,
            Outcome::Lose => 0 + 1,
        },
        Shape::Scissor => match outcome {
            Outcome::Win => 6 + 1,
            Outcome::Draw => 3 + 3,
            Outcome::Lose => 0 + 2,
        },
    }
}

enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}
