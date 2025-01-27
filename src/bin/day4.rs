use aoc_rust::inputs;
use std::{collections::{HashMap, HashSet}, u64};

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

//part1 alternative
// fn main()->inputs::Result<()> {
//     let input=inputs::fetch(2020, 4)?;
//     let valid_pass=parse(&input).iter().filter(|pass|is_valid(pass)).count();
//     println!("{valid_pass}");
//     Ok(())
// }


//part2
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 4)?;
    let valid_pass=parse(&input).iter().filter(|pass|is_valid2(pass)).count();
    println!("{valid_pass}");
    Ok(())
}

fn in_range(field:&str,min:u32,max:u32)->bool {
    match  field.parse::<u32>(){
        Ok(field) => field>=min && field<=max,
        Err(_) => false,
    }
}

fn valid_height(field:&str,)->bool {
    if field.ends_with("cm") {
        let num=field.strip_suffix("cm").unwrap();
        in_range(num, 150, 193)
    }
    else if field.ends_with("in") {
        let num=field.strip_suffix("in").unwrap();
        in_range(num, 59, 76)
    }
    else {
        false
    }
}

fn valid_ecl(field:&str)->bool {
    const COLORS:[&str;7]=["amb","blu","brn","gry","grn","hzl","oth"];
    COLORS.contains(&field)
}

fn valid_pid(field:&str)->bool {
    if field.chars().all(|c|c.is_ascii_digit()) && field.len()==9 {
        true
    }
    else {
        false
    }
}

fn valid_hair_color(field:&str)->bool {
    let field=field.strip_prefix("#");
    match field {
        Some(field)=> {
            if field.chars().all(|c| c.is_ascii_lowercase()||c.is_ascii_digit()) {
                true
            }
            else {
                false
            }
        }
        None=>false,
    }
}

#[cfg(test)]
mod tests {


    #[test]
    fn hair_color() {
        let result=crate::valid_hair_color("#425ag5");
        assert_eq!(result,true);
    }
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
        let valid_byr=in_range(passport.get("byr").unwrap(), 1920, 2002);
        let valid_iyrr=in_range(passport.get("iyr").unwrap(), 2010, 2020);
        let valid_eyr=in_range(passport.get("eyr").unwrap(), 2020, 2030);
        let valid_hgt=valid_height(passport.get("hgt").unwrap());
        let valid_hcl=valid_hair_color(passport.get("hcl").unwrap());
        let valid_ecl=valid_ecl(passport.get("ecl").unwrap());
        let valid_pid=valid_pid(passport.get("pid").unwrap());
        return valid_byr && valid_iyrr && valid_eyr && valid_hgt && valid_hcl && valid_ecl && valid_pid
    }
    false
}