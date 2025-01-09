pub mod inputs;

fn main() -> inputs::Result<()> {
    let input=inputs::fetch(2024,3 )?;
    println!("{}",input);
    Ok(())
}
