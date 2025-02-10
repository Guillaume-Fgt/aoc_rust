use aoc_rust::inputs;
use itertools::Itertools;

fn main() -> inputs::Result<()> {
    let input = inputs::fetch(2020, 10)?;
    let mut input: Vec<i32> = input.lines().map(|n| n.parse::<i32>().unwrap()).collect();
    input.sort();
    let mut one_jolt = 0;
    let mut three_jolt = 0;
    for (a, b) in input.iter().tuple_windows() {
        let dif = b - a;
        println!("{b}-{a}={}", b - a);
        match dif {
            1 => one_jolt += 1,
            3 => three_jolt += 1,
            _ => (),
        }
    }
    println!("{}", one_jolt);
    println!("{}", three_jolt);
    //we had one to take into account start and finish differences.
    println!("{}", (one_jolt + 1) * (three_jolt + 1));
    // let device_jolt = input.max().unwrap() + 3;
    Ok(())
}
