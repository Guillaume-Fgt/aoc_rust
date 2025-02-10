use aoc_rust::inputs;
use std::collections::HashSet;
fn main() -> inputs::Result<()> {
    let input = inputs::fetch(2020, 9)?;
    let input = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut start = 0;
    let mut end = 25;

    while end <= input.len() - 1 {
        let nums = &input[start..end];
        let curr = &input[end];
        let valid = nums
            .into_iter()
            .any(|e| nums.contains(&(curr - e)) && curr - e != *e);
        if !valid {
            println!("{curr}");
            break;
        }
        start += 1;
        end += 1;
    }

    Ok(())
}
