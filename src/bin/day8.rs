use aoc_rust::inputs;

fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 8)?;
    let mut instr=Vec::new();
    let mut total=0;
    for (inst,num) in input.lines().map(|l|l.split_once(" ").unwrap()) {
        instr.push((inst,num.parse::<i32>().unwrap())); 
        
    }
    
    let mut curr_index:i32=0;
    let mut visited=Vec::new();


    loop {
        let current=instr.get(curr_index as usize).unwrap();
        if visited.contains(&curr_index) {
            break;
        }
        visited.push(curr_index);
        println!("{:?}",visited);
        match current.0 {
            "acc"=>{
                total+=current.1;
                curr_index+=1;
            }
            "jmp"=>{
                curr_index+=current.1 as i32;
            }
            "nop"=>{
                curr_index+=1;
            }
            "viewed"=>{
                break;
            }
            _=>{}
        }
    }
    println!("{total}");
    Ok(())
}




