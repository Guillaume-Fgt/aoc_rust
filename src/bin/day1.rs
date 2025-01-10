use aoc_rust::inputs;

fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020,1 )?;
    let mut nums=input.split("\n").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    
    //naive solution
    // loop {
    //     let num=nums.pop().ok_or("No more numbers")??;
    //     for num2 in nums.iter() {
    //         let num2=num2.as_ref().unwrap();
    //         for num3 in nums.iter() {
    //             let num3=num3.as_ref().unwrap();
    //             if num+num2+num3==2020 {
    //                 println!("found!");
    //                 println!("{}*{}*{}={}",num,num2,num3,num*num2*num3);
    //                 break;
    //             }
    //         }
    //     }
    // }

    //O(NlogN) solution part1
    nums.sort();
    for i in 0..nums.len() {
        if let Ok(j)=nums.binary_search(&(2020-nums[i])) {
            if i!=j {
                println!("{}",nums[i]*nums[j]);
                break;
            }
        }
    }
    Ok(())
}