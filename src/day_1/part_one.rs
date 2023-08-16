#[allow(dead_code)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("src/day_1/input.txt")?;

    let mut largest = 0;
    let mut current_sum = 0;

    file.split('\n')
        .map(|split| split.parse::<i32>())
        .for_each(|split| match split {
            Ok(number) => current_sum += number,
            Err(_) => {
                if current_sum > largest {
                    largest = current_sum;
                }
                current_sum = 0;
            }
        });

    dbg!(largest);
    Ok(())
}
