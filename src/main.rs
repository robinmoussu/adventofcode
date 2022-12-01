use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input-01-1.txt")?;
    let reader = BufReader::new(file);

    let mut all_max: Vec<i64> =
        reader
        .lines()
        .flat_map(|x| x)
        .group_by(|e| !e.is_empty())
        .into_iter()
        .flat_map(|(key, group)| key.then(||group))
        .map(|group| group
            .map(|x| x.parse::<i64>().unwrap())
            .sum()
        )
        .collect();
    all_max.sort_by(|lhs, rhs| rhs.cmp(lhs)); // Sort by decreasing order

    let top_3: i64 = all_max.iter().take(3).sum();

    println!("day 1.2: {:?}", top_3);

    Ok(())
}
