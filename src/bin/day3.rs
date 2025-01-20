use aoc_rust::inputs;

fn main()-> inputs::Result<()>{
    let input=inputs::fetch(2020, 3)?;
    let mut grid:Vec<Vec<char>>=Vec::new();
    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }
    let slopes=[(1,1),(3,1),(5,1),(7,1),(1,2)];
    let mut answer=Vec::new();
    for slope in slopes {
        answer.push(count_trees(&grid, slope));
    }
    println!("{:?}",answer.iter().copied().into_iter().reduce(|a,b| a*b).unwrap());
    Ok(())
}

fn count_trees(grid:&Vec<Vec<char>>,mvt:(usize,usize))->i32 {
    let width=grid[0].len();
    let height=grid.len();
    let mut i=0;
    let mut j=0;
    let mut total_tree=0;
    loop {
        i+=mvt.0;
        j+=mvt.1;
        if j>height-1 {
            break;
        }
        if grid[j][i%width]=='#' {
            total_tree+=1;
        }
    }
    total_tree

}