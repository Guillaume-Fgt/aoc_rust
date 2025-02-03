use std::collections::{HashMap, HashSet};


use aoc_rust::inputs;
use regex::Regex;
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 7)?;
    let re=Regex::new(r"(?:(\d)\s)(\w+\s\w+) bags?").unwrap();
    let mut bag_types:HashMap<&str,HashMap<&str,u32>>=HashMap::new();
    for line in input.lines() {
        let bag:Vec<&str>=line.splitn(2, " bags").collect();
        let entry=bag_types.entry(bag[0]).or_insert(HashMap::new());
        for (_,[num,name]) in re.captures_iter(line).map(|caps|caps.extract()) {
            entry.insert(name,num.trim().parse::<u32>().unwrap());
            }
    }

    let mut queue=Vec::new();
    let mut valid:HashSet<&str>=HashSet::new();
    queue.push("shiny gold");

    while !queue.is_empty() {
        let current=queue.pop().unwrap();
        for (k,v) in bag_types.iter() {
            if v.contains_key(&current) {
                queue.push(k);
                valid.insert(k);
            }
        }
    }
    println!("{:?}",valid);
    println!("{:?}",valid.len());
    Ok(())
}

