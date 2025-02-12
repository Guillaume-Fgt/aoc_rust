use aoc_rust::inputs;
use itertools::Itertools;

//part1
// fn main() -> inputs::Result<()> {
//     let input = inputs::fetch(2020, 10)?;
//     let mut input: Vec<i32> = input.lines().map(|n| n.parse::<i32>().unwrap()).collect();
//     input.sort();
//     let mut one_jolt = 0;
//     let mut three_jolt = 0;
//     for (a, b) in input.iter().tuple_windows() {
//         let dif = b - a;
//         println!("{b}-{a}={}", b - a);
//         match dif {
//             1 => one_jolt += 1,
//             3 => three_jolt += 1,
//             _ => (),
//         }
//     }
//     println!("{}", one_jolt);
//     println!("{}", three_jolt);
//     //we had one to take into account start and finish differences.
//     println!("{}", (one_jolt + 1) * (three_jolt + 1));
//     Ok(())
// }

//part2

fn main() -> inputs::Result<()> {
    let input = inputs::fetch(2020, 10)?;
    let mut input: Vec<i32> = input.lines().map(|n| n.parse::<i32>().unwrap()).collect();
    input.push(0);
    input.push(input.iter().max().unwrap()+3);
    input.sort();
    println!("{:?}",input);
    let mut total:i64=1;
    let mut one_jolt:Vec<u64> = Vec::with_capacity(10);
    for (a, b) in input.iter().tuple_windows() {
        let dif = b - a;
        println!("{b}-{a}={}", b - a);
        match dif {
            1 => {
                one_jolt.push(1);
            },
            _=>{
                if !one_jolt.is_empty() {
                    let power:u32=one_jolt.len().try_into().unwrap();
                    one_jolt.clear();
                    if power==4 {
                        total*=7;
                        println!("{}",total);
                    }
                    else if power==3 {
                        total*=4;
                        println!("{}",total);
                    }
                    else if power==2 {
                        total*=2;
                        println!("{}",total);
                    }
                }

            },
        }
    }

    println!("{}",total);

    Ok(())
}
