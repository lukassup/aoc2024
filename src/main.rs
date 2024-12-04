use std::io;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> io::Result<()> {
    day1::part1("input/day1.txt").map(|n| println!("day1p1: {n}"))?;
    day1::part2("input/day1.txt").map(|n| println!("day1p2: {n}"))?;
    day2::part1("input/day2.txt").map(|n| println!("day2p1: {n}"))?;
    day2::part2("input/day2.txt").map(|n| println!("day2p2: {n}"))?;
    day3::part1("input/day3.txt").map(|n| println!("day3p1: {n}"))?;
    day3::part2("input/day3.txt").map(|n| println!("day3p2: {n}"))?;
    day4::part1("input/day4.txt").map(|n| println!("day4p1: {n}"))?;
    day4::part2("input/day4.txt").map(|n| println!("day4p2: {n}"))?;
    Ok(())
}
