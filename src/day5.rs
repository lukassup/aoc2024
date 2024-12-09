use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::{fs::File, io, path::Path};

pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<u32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut lines = buf.lines().map_while(Result::ok);

    let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();
    lines
        .by_ref()
        // until first empty line
        .take_while(|s| !s.is_empty())
        .for_each(|line| {
            let Some((num, ord)) = line.split_once('|') else {
                return;
            };
            let Some(num) = num.parse::<u32>().ok() else {
                return;
            };
            let Some(ord) = ord.parse::<u32>().ok() else {
                return;
            };
            ordering.entry(num).or_default().push(ord);
        });

    let updates: Vec<Vec<u32>> = lines
        .by_ref()
        .map(|line| {
            line.split(',')
                .flat_map(|num| num.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut result: u32 = 0;
    'out: for update in updates.iter() {
        for (pos_upd, num_upd) in update.iter().enumerate() {
            let order = ordering.get(num_upd).expect("no ordering for number");
            let before = &update[0..pos_upd];
            if order.iter().any(|ord_num| before.contains(ord_num)) {
                continue 'out;
            }
        }
        result += update[update.len() / 2];
    }
    Ok(result)
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<u32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut lines = buf.lines().map_while(Result::ok);

    let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();
    lines
        .by_ref()
        // until first empty line
        .take_while(|s| !s.is_empty())
        .for_each(|line| {
            let Some((num, ord)) = line.split_once('|') else {
                return;
            };
            let Some(num) = num.parse::<u32>().ok() else {
                return;
            };
            let Some(ord) = ord.parse::<u32>().ok() else {
                return;
            };
            ordering.entry(num).or_default().push(ord);
        });

    let mut updates: Vec<Vec<u32>> = lines
        .by_ref()
        .map(|line| {
            line.split(',')
                .flat_map(|num| num.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut swaps: Vec<Vec<u32>> = Vec::new();
    'out: for update in updates.into_iter() {
        let mut swapped = false;
        for (pos_upd, num_upd) in update.iter().enumerate() {
            let nums_before = ordering.get_mut(num_upd).expect("no ordering for number");
            nums_before.sort();
            for num_before in nums_before.iter() {
                let Some(pos_before) = update.iter().position(|n| n == num_before) else {
                    continue;
                };
                if pos_upd < pos_before {
                    continue;
                }
                // dbg!(num_upd, &nums_before);
                swapped = true;
            }
        }
        if swapped {
            swaps.push(update);
            break 'out;
        }
    }

    let mut result: u32 = 0;
    for swap in swaps {
        result += swap[swap.len()/2];
    }
    Ok(result)
}
