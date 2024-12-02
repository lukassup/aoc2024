use std::io;

mod day1;
mod day2;

fn main() -> io::Result<()> {
    day1::part1("input/day1.txt")?;
    day1::part2("input/day1.txt")?;
    day2::part1("input/day2.txt")?;
    day2::part2("input/day2.txt")?;
    Ok(())
}
