use std::{cmp::Ordering, collections::HashSet, error::Error};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_9/input.txt")?;

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { ..head_position };

    let mut visited_tail_positions: HashSet<Position> = HashSet::new();
    visited_tail_positions.insert(tail_position);

    input
        .lines()
        .map(|line| {
            let splitted_line: Vec<&str> = line.split(' ').collect();
            (
                splitted_line[0].chars().next().unwrap(),
                splitted_line[1].parse().unwrap_or(0),
            )
        })
        .for_each(|(direction, steps)| {
            (0..steps).for_each(|_| {
                let new_head_position = get_next_head_position(&head_position, &direction);
                head_position = new_head_position;

                if let Some(new_tail_position) =
                    get_next_tail_position(&tail_position, &head_position)
                {
                    tail_position = new_tail_position;
                    visited_tail_positions.insert(tail_position);
                }
            });
        });

    dbg!(visited_tail_positions.len());

    Ok(())
}

fn get_next_head_position(starting_position: &Position, direction: &char) -> Position {
    let mut new_position = Position {
        ..*starting_position
    };

    match direction {
        'U' => new_position.y += 1,
        'D' => new_position.y -= 1,
        'R' => new_position.x += 1,
        'L' => new_position.x -= 1,
        _ => panic!(),
    };

    new_position
}

fn get_next_tail_position(tail: &Position, head: &Position) -> Option<Position> {
    let mut new_position = Position { ..*tail };

    if head.x - tail.x > 1 {
        match head.y.cmp(&tail.y) {
            Ordering::Equal => {
                new_position.x += 1;
            }
            Ordering::Less => {
                new_position.y -= 1;
                new_position.x += 1;
            }
            Ordering::Greater => {
                new_position.y += 1;
                new_position.x += 1;
            }
        }
    } else if tail.x - head.x > 1 {
        match head.y.cmp(&tail.y) {
            Ordering::Equal => {
                new_position.x -= 1;
            }
            Ordering::Less => {
                new_position.y -= 1;
                new_position.x -= 1;
            }
            Ordering::Greater => {
                new_position.y += 1;
                new_position.x -= 1;
            }
        }
    } else if head.y - tail.y > 1 {
        match head.x.cmp(&tail.x) {
            Ordering::Equal => {
                new_position.y += 1;
            }
            Ordering::Less => {
                new_position.x -= 1;
                new_position.y += 1;
            }
            Ordering::Greater => {
                new_position.x += 1;
                new_position.y += 1;
            }
        }
    } else if tail.y - head.y > 1 {
        match head.x.cmp(&tail.x) {
            Ordering::Equal => {
                new_position.y -= 1;
            }
            Ordering::Less => {
                new_position.x -= 1;
                new_position.y -= 1;
            }
            Ordering::Greater => {
                new_position.x += 1;
                new_position.y -= 1;
            }
        }
    } else {
        return None;
    }

    Some(new_position)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
