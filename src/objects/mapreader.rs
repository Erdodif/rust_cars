#[allow(dead_code,unused,unused_attributes)]
use std::{fs::File};
use std::io::{Error,BufReader, BufRead};
use super::map::Position;

pub fn get_map(path:&str) -> Result<Vec<Vec<Position>>,Error>{
    let mut acc :Vec<Vec<Position>>= Vec::new();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    loop{
        str.clear();
        if reader.read_line(&mut str)? == 0 {
            return Ok(acc)
        }
        let positions = str.trim().split(' ').map(|str|{str.into()}).collect();
        acc.push(positions);
    }
}