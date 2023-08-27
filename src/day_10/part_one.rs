use std::error::Error;

#[allow(unused)]
pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/day_10/input.txt")?;

    let mut cpu = structs::Cpu::new();

    input.lines().for_each(|line| {
        let args: Vec<&str> = line.split(' ').collect();

        match *args.first().unwrap() {
            "noop" => cpu.tick(),
            "addx" => {
                cpu.tick();
                cpu.tick();
                cpu.modify_register(args.last().unwrap().parse::<i32>().unwrap_or(0));
            }
            _ => panic!(),
        }
    });

    dbg!(cpu);

    Ok(())
}

pub mod structs {
    #[derive(Debug)]
    pub struct Cpu {
        register: i32,
        cycle_count: u32,

        sum: i32,
    }

    impl Cpu {
        pub fn new() -> Self {
            Self {
                register: 1,
                cycle_count: 0,
                sum: 0,
            }
        }
        pub fn tick(&mut self) {
            self.cycle_count += 1;

            if self.cycle_count % 40 == 20 {
                self.sum += self.cycle_count as i32 * self.register;
            }
        }
        pub fn modify_register(&mut self, value: i32) {
            self.register += value;
        }
    }
}
