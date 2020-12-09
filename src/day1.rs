use std::{collections::BTreeSet, str::FromStr};

pub fn day1_easy(lines: Vec<String>) {
    let (lower, higher): (BTreeSet<_>, BTreeSet<_>) = lines
        .iter()
        .filter_map(|line| u32::from_str(line).ok())
        .partition(|num| num < &1010);

    let compares: BTreeSet<_> = lower.iter().map(|number| 2020 - number).collect();
    let common = compares.intersection(&higher);

    println!("day 1 solutions:");
    for value in common {
      println!("num 1: {}, num 2: {}, product: {}", value, 2020 - value, value * (2020 - value));
    }
}
