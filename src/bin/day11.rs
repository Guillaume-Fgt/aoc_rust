use aoc_rust::inputs;
use itertools::iproduct;

fn main() -> inputs::Result<()> {
    let input = inputs::fetch(2020, 11)?;
    let mut input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut to_swap = Vec::new();
    loop {
        to_swap.clear();
        for (i, j) in iproduct!(0..input.len(), 0..input[0].len()) {
            if input[i][j] != '.' && should_swap(&input, i, j) {
                to_swap.push((i, j));
            }
        }
        for &(i, j) in &to_swap {
            input[i][j] = if input[i][j] == 'L' { '#' } else { 'L' };
        }
        if to_swap.is_empty() {
            break;
        }
    }
    let result = input.iter().flatten().filter(|&&c| c == '#').count();
    println!("{}", result);
    Ok(())
}

fn should_swap(input: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let neighbors = get_neighbors(i, j, input);
    match input[i][j] {
        'L' => neighbors.iter().all(|&c| c != '#'),
        '#' => neighbors.iter().filter(|&&c| c == '#').count() >= 4,
        _ => false,
    }
}

fn get_neighbors(row: usize, col: usize, grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut neighbors: Vec<char> = Vec::with_capacity(8);
    for r in row.saturating_sub(1)..=row + 1 {
        for c in col.saturating_sub(1)..=col + 1 {
            if r == row && c == col {
                continue;
            } else if r < grid.len() && c < grid[0].len() {
                neighbors.push(grid[r][c]);
            }
        }
    }
    neighbors
}
