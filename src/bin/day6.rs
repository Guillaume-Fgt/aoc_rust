use std::collections::HashSet;

use aoc_rust::inputs;

fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 6)?;
    let mut total=0;
    for answer in input.split("\n\n") {
        let answers=answer.split("\n").map(|a|a.chars());
        let mut set=HashSet::new();
        for a in answers {
            set.extend(a);
        }
        total+=set.len();
    }
    print!("Total: {}",total);
    Ok(())
}
