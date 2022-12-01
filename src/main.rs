use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input-01-1.txt")?;
    let reader = BufReader::new(file);

    let max: i64 = 
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
        .max()
        .unwrap();

    println!("day 1.1: {:?}", max);

    Ok(())
}
