mod objects;
use crate::objects::{map::*, mapreader,car::Car};
use objects::coordinates::Coordinate2D as Coordinate;
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
fn main() -> Result<(), std::io::Error> {
    let mut line = String::new();
    print!("{}", "File path: ");
    stdout().flush()?;
    stdin().read_line(&mut line)?;
    let positions = mapreader::get_map(&line.trim())?;
    let mut game = Game::new(positions, Vec::new())?;
    print!("{}", "Car count: ");
    stdout().flush()?;
    line.clear();
    stdin().read_line(&mut line)?;
    print!("{}",line);
    let mut x = line.trim().parse::<u8>().expect("F");
    println!("{}",game);
    while x != 0 {
        println!("{:?}",game.cars());
        let mut rng = thread_rng();
        let pos = Coordinate::new(
            rng.gen_range(1..=(game.boundary().x)),
            rng.gen_range(1..=(game.boundary().y)),
        );
        if game.position(&pos).unwrap_or(Position::Wall) == Position::Road {
            game.add_car(Car::from(pos));
            x -= 1;
        }
    }
    println!("{}",game);
    Ok(())
}
