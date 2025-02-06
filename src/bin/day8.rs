use aoc_rust::inputs;

//part1
// fn main()->inputs::Result<()> {
//     let input=inputs::fetch(2020, 8)?;
//     let mut instr=Vec::new();
//     let mut total=0;
//     for (inst,num) in input.lines().map(|l|l.split_once(" ").unwrap()) {
//         instr.push((inst,num.parse::<i32>().unwrap())); 
        
//     }
    
//     let mut curr_index:i32=0;
//     let mut visited=Vec::new();


//     loop {
//         let current=instr.get(curr_index as usize).unwrap();
//         if visited.contains(&curr_index) {
//             break;
//         }
//         visited.push(curr_index);
//         println!("{:?}",visited);
//         match current.0 {
//             "acc"=>{
//                 total+=current.1;
//                 curr_index+=1;
//             }
//             "jmp"=>{
//                 curr_index+=current.1 as i32;
//             }
//             "nop"=>{
//                 curr_index+=1;
//             }
//             _=>{}
//         }
//     }
//     println!("{total}");
//     Ok(())
// }

//part2
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 8)?;
    let mut instr=Vec::new();
    for (inst,num) in input.lines().map(|l|l.split_once(" ").unwrap()) {
        instr.push((inst,num.parse::<i32>().unwrap())); 
        
    }
    let mut visited:Vec<(i32,bool)>=Vec::new();
    let mut queue:Vec<(i32,i32,bool)>=Vec::new();
    queue.push((0,0,false));
    let goal=instr.len() as i32;

    while !queue.is_empty() {
        println!("{:?}",queue);
        let (curr_index,mut total,already_changed):(i32,i32,bool)=queue.pop().unwrap();
        println!("{curr_index}{total}");
        
        if curr_index==goal {
            println!("Success!");
            println!("The total is: {total}");
            break;
        }
        let current=instr.get(curr_index as usize).unwrap();
        if visited.contains(&(curr_index,already_changed)) {
            continue;
        }
        visited.push((curr_index,already_changed));
        match current.0 {
            "acc"=>{
                total+=current.1;
                queue.push((curr_index+1,total,already_changed));
            }
            "jmp"=>{
                queue.push((curr_index+current.1 as i32,total,already_changed));
                if !already_changed {
                    queue.push((curr_index+1,total,true));
                }
            }
            "nop"=>{
                queue.push((curr_index+1,total,already_changed));
                if !already_changed {
                    queue.push((curr_index+current.1 as i32,total,true));
                } 
            }
            _=>{}
        }
    }
    Ok(())
}

