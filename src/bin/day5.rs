
use aoc_rust::inputs;

// //part1
// fn main()->inputs::Result<()> {
//     let input=inputs::fetch(2020, 5)?;
//     let mut sum=0;

//     for (instr_row,instr_col) in parse(&input) {
//         let row=compute_place(instr_row,0,127,'F');
//         let col=compute_place(instr_col, 0, 7,'L');
//         sum=max(sum,row*8+col);
//     }
//     print!("{sum}");
//     Ok(())
// }

//part2
fn main()->inputs::Result<()> {
    let input=inputs::fetch(2020, 5)?;
    let mut seat_ids=Vec::new();

    for (instr_row,instr_col) in parse(&input) {
        let row=compute_place(instr_row,0,127,'F');
        let col=compute_place(instr_col, 0, 7,'L');
        seat_ids.push(row*8+col);
    }
    for i in 0..890 {
        if !seat_ids.contains(&i) {
            println!("{i}");
        }
    }
    Ok(())
}


fn compute_place(instr:&str,left:i32,right:i32,lett:char)->i32 {
    let mut l:i32=left;
    let mut r:i32=right;

    for letter in instr.chars() {
        if letter==lett {
            if r-l==1 {
                return l;
            }
            r=(r-l).div_euclid(2)+l;
        }
        else {
            if r-l==1 {
                return r;
            }
            l=(r-l).div_euclid(2)+1+l;
        }
    }
    0
}

fn parse(input:&str)-> impl Iterator<Item = (&str, &str)>{
    input.split("\n").map(|e|e.split_at(7))
}

#[test]
fn parse_input() {
    let mut result=parse(
        "FFBBFFFLLL
                BFBFBFBLLR");
    assert_eq!(result.next(),Some(("FFBBFFF","LLL")));
}

#[test]
fn row() {
    let row=compute_place("FBFBBFF",0,127,'F');
    assert_eq!(row,44);
}

#[test]
fn col() {
    let row=compute_place("RLR",0,7,'L');
    assert_eq!(row,5);
}