use aoc_rust::inputs;

fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020,2 )?;
    let mut valid=0;
    for line in input.lines(){
        let mut iter=line.split_whitespace();
        let mut range=iter.next().ok_or("no item range")?.split("-");
        let lower_bound=range.next().ok_or("no lower bound")?.parse::<u32>()?;
        let upper_bound=range.next().ok_or("no upper bound")?.parse::<u32>()?+1;
        let letter=iter.next().ok_or("no item letter")?.chars().next().ok_or("no char")?;
        let word=iter.next().ok_or("no item word")?;
        let r=lower_bound..upper_bound;
        if r.contains(&count_letter(&word, letter).unwrap()) {
            valid+=1;
        }
    }
    println!("{}",valid);
    Ok(())
}

fn count_letter(word:&str,letter:char)->inputs::Result<u32>{
    Ok(u32::try_from(word.chars().filter(|x| x.to_ascii_lowercase()==letter).count())?)
}