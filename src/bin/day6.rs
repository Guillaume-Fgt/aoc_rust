use std::collections::HashSet;

use aoc_rust::inputs;

// //part1
// fn main()->inputs::Result<()> {
//     let input=inputs::fetch(2020, 6)?;
//     let mut total=0;
//     for answer in input.split("\n\n") {
//         let answers=answer.split("\n").map(|a|a.chars());
//         let mut set=HashSet::new();
//         for a in answers {
//             set.extend(a);
//         }
//         total+=set.len();
//     }
//     print!("Total: {}",total);
//     Ok(())
// }


//part2
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 6)?;
    let mut sum=0;

    for answer in input.split("\n\n") {
        let mut group_a:Vec<HashSet<u8>>=Vec::new();
        for a in answer.split("\n") {
            let mut set = HashSet::new();
            set.extend(a.bytes());
            group_a.push(set);
        }
        let total=group_a.into_iter().reduce(|a,b|a.intersection(&b).cloned().collect()).ok_or("error")?;
        sum+=total.len();
    }
    print!("{sum}");
    Ok(())
}