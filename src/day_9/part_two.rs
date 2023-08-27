use std::{cell::RefCell, cmp::Ordering, collections::HashSet, error::Error};

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_9/input.txt")?;

    let mut knots: Vec<RefCell<Position>> = vec![RefCell::new(Position { x: 0, y: 0 }); 10];

    let mut visited_tail_positions: HashSet<Position> = HashSet::new();
    visited_tail_positions.insert(*knots.last().unwrap().borrow());

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
                move_head(&mut knots.first().unwrap().borrow_mut(), &direction);

                knots.iter().skip(1).enumerate().for_each(|(i, knot)| {
                    set_knot_position(&mut knot.borrow_mut(), &knots[i].borrow());
                });

                visited_tail_positions.insert(*knots.last().unwrap().borrow());
            });
        });

    dbg!(visited_tail_positions.len());

    Ok(())
}

fn move_head(head: &mut Position, direction: &char) {
    let mut new_position = Position { ..*head };

    match direction {
        'U' => new_position.y += 1,
        'D' => new_position.y -= 1,
        'R' => new_position.x += 1,
        'L' => new_position.x -= 1,
        _ => panic!(),
    };

    *head = new_position;
}

fn set_knot_position(tail: &mut Position, head: &Position) {
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
        return;
    }

    *tail = new_position;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
