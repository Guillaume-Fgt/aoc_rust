use std::fs;
use std::env;
use reqwest;
pub type Result<T>=core::result::Result<T,Error>;
pub type Error=Box<dyn std::error::Error>;

const DIR:&str="inputs";

fn file_exists(path:&str)->bool {
    fs::metadata(path).is_ok()
}

pub fn fetch(year:u16,day:u8)  -> Result<String> {
    let path=format!("{DIR}/{}/{:02}.txt",year,day);
    create_dir(year)?;
    if file_exists(&path) {
        println!("File already exists, reading from file");
        let input=read_file(&path)?;
        Ok(input)
    }
    else {
        println!("File does not exist,downloading from AOC website");
        let url=format!("https://adventofcode.com/{}/day/{}/input",year,day);
        let token=env::var("AOC_TOKEN")?;
        let input=get_input(&url, &token)?;
        if save_to_file(&path, &input).is_ok() {
            let input=read_file(&path)?;
            Ok(input)
        }
        else {
            Err("Failed to fetch input in txt file".into())
        }
    }
}

fn read_file(path:&str)->Result<String> {
    let input=fs::read_to_string(path)?;
    Ok(input)
}

///Download the input from Aoc website
fn get_input(url:&str,token:&str)->Result<String>{
    let client=reqwest::blocking::Client::new();
    let input=client.get(url)
        .header("Cookie",format!("session={}",token))
        .send()?
        .text()?;
    Ok(input)
}

///Save input text to a file
fn save_to_file(path:&str,input:&str)->Result<()> {
    fs::write(path, input.trim_end())?;
    Ok(())
}

fn create_dir(year:u16) ->Result<()>{
    fs::create_dir_all(format!("{DIR}/{}",year))?;
    Ok(())
}