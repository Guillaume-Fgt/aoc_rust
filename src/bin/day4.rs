use aoc_rust::inputs;
use std::collections::HashMap;
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 4)?;
    let passports=input.split("\n\n");
    const REQUIRED_FIELDS: [&str; 7]=["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    let mut valid=0;
    for passport in passports {
        valid+=1;
        let sanitize=str::replace(passport, "\n", " ");
        let mut hash_pass:HashMap<&str, &str>=HashMap::new();
        for elem in sanitize.split(" ").collect::<Vec<&str>>() {
            let e=elem.split(":").collect::<Vec<&str>>();
            hash_pass.insert(e.first().unwrap(), e.last().unwrap());
        }
        for field in REQUIRED_FIELDS {
            if hash_pass.get(field)==None {
                valid-=1;
                break;
            }
        }
    println!("{}",valid);
    }
    Ok(())
}