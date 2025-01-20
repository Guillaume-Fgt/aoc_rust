use aoc_rust::inputs;

fn main()-> inputs::Result<()>{
    let input=inputs::fetch(2020, 3)?;
    let mut grid:Vec<Vec<char>>=Vec::new();
    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }
    let width=grid[0].len();
    let height=grid.len();
    let mut i=0;
    let mut j=0;
    let mut total_tree=0;
    loop {
        i+=3;
        j+=1;
        if j>height-1 {
            break;
        }
        if grid[j][i%width]=='#' {
            total_tree+=1;
        }
    }
    println!("{total_tree}");
    Ok(())
}