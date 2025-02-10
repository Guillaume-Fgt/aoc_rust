use aoc_rust::inputs;

//part1

// fn main() -> inputs::Result<()> {
//     let input = inputs::fetch(2020, 9)?;
//     let input = input
//         .lines()
//         .map(|n| n.parse::<i64>().unwrap())
//         .collect::<Vec<i64>>();

//     let mut start = 0;
//     let mut end = 25;

//     while end <= input.len() - 1 {
//         let nums = &input[start..end];
//         let curr = &input[end];
//         let valid = nums
//             .into_iter()
//             .any(|e| nums.contains(&(curr - e)) && curr - e != *e);
//         if !valid {
//             println!("{curr}");
//             break;
//         }
//         start += 1;
//         end += 1;
//     }

//     Ok(())
// }

//part2

fn main() -> inputs::Result<()> {
    let input = inputs::fetch(2020, 9)?;
    let input = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let invalid_num=1038347917;
    let mut sum:i64=0;
    let mut origin_index=0;
    let mut index=0;
    let mut nums:Vec<i64>=Vec::new();

    loop {
        let num=input.get(index).unwrap();
        sum+=num;
        nums.push(*num);
        if sum==invalid_num {
            println!("solved!");
            println!("{:?}",nums.iter().min());
            println!("{:?}",nums.iter().max());
            println!("{:?}",nums.iter().min().unwrap()+nums.iter().max().unwrap());
            break;
        }
        if sum>invalid_num {
            origin_index+=1;
            index=origin_index;
            sum=0;
            nums.clear();
            continue;
        }
        index+=1;
    }


    Ok(())
}