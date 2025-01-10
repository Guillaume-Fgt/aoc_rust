use aoc_rust::inputs;

fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020,1 )?;
    let mut nums:Vec<Result<u32, std::num::ParseIntError>>=input.split("\n").map(|x| x.parse::<u32>()).collect();
    loop {
        let num=nums.pop().ok_or("No more numbers")??;
        for num2 in nums.iter() {
            let num2=num2.as_ref().unwrap();
            for num3 in nums.iter() {
                let num3=num3.as_ref().unwrap();
                if num+num2+num3==2020 {
                    println!("found!");
                    println!("{}*{}*{}={}",num,num2,num3,num*num2*num3);
                    break;
                }
            }
        }
    }
}