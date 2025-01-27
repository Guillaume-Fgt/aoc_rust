use aoc_rust::inputs;
use std::collections::{HashMap, HashSet};

//part1
// fn main()->inputs::Result<()> {
//     let input=inputs::fetch(2020, 4)?;
//     let passports=input.split("\n\n");
//     const REQUIRED_FIELDS: [&str; 7]=["byr","iyr","eyr","hgt","hcl","ecl","pid"];
//     let mut valid=0;
//     for passport in passports {
//         valid+=1;
//         let sanitize=str::replace(passport, "\n", " ");
//         let mut hash_pass:HashMap<&str, &str>=HashMap::new();
//         for elem in sanitize.split(" ").collect::<Vec<&str>>() {
//             let e=elem.split(":").collect::<Vec<&str>>();
//             hash_pass.insert(e.first().unwrap(), e.last().unwrap());
//         }
//         for field in REQUIRED_FIELDS {
//             if hash_pass.get(field)==None {
//                 valid-=1;
//                 break;
//             }
//         }
//     println!("{}",valid);
//     }
//     Ok(())
// }

//part2
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 4)?;
    let valid_pass=parse(&input).iter().filter(|pass|is_valid(pass)).count();
    println!("{valid_pass}");
    // let passports=input.split("\n\n");
    // const REQUIRED_FIELDS: [&str; 7]=["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    // let mut valid=0;
    // for passport in passports {
    //     valid+=1;
    //     let sanitize=str::replace(passport, "\n", " ");
    //     let mut hash_pass:HashMap<&str, &str>=HashMap::new();
    //     for elem in sanitize.split(" ").collect::<Vec<&str>>() {
    //         let e=elem.split(":").collect::<Vec<&str>>();
    //         hash_pass.insert(e.first().unwrap(), e.last().unwrap());
    //     }
    //     for field in REQUIRED_FIELDS {
    //         if hash_pass.into_keys()
    //     }
    
    // println!("{}",valid);
    // }
    Ok(())
}

fn valid_year(field:&str,min:u32,max:u32)->bool {
    match  field.parse::<u32>(){
        Ok(field) => field>=min && field<=max,
        Err(_) => false,
    }
}

fn valid_height(field:&str,)->bool {
    
}

fn parse(input:&str)->Vec<HashMap<String, String>> {
    input.split("\n\n").map(|passport| {
        passport.split_ascii_whitespace().map(|kv|{
            let mut kv=kv.split(":");
            let key=kv.next().unwrap().to_string();
            let value=kv.next().unwrap().to_string();
            (key,value)
        })
        .collect::<HashMap<_,_>>()
    })
    .collect::<Vec<_>>()
}

fn is_valid(passport:&HashMap<String,String>)->bool {
    let keys=passport.keys().collect::<HashSet<_>>();
    const REQUIRED_FIELDS: [&str; 7]=["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    REQUIRED_FIELDS.iter().all(|field| keys.contains(&field.to_string()))
}

fn is_valid2(passport:&HashMap<String,String>)->bool {
    if is_valid(passport) {
        let valid_byr=valid_year(passport.get("byr").unwrap(), 1920, 2002);
        let valid_iyrr=valid_year(passport.get("iyr").unwrap(), 2010, 2020);
        let valid_eyr=valid_year(passport.get("eyr").unwrap(), 2020, 2030);
    }
}