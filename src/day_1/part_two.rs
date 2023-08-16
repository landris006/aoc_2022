#[allow(dead_code)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("src/day_1/input.txt")?;

    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum = 0;

    file.split("\n")
        .into_iter()
        .map(|split| split.parse::<i32>().ok())
        .for_each(|split| match split {
            Some(number) => current_sum += number,
            None => {
                sums.push(current_sum);
                current_sum = 0;
            }
        });

    sums.sort();
    dbg!(sums.iter().rev().take(3).sum::<i32>());

    Ok(())
}
