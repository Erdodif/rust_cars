#[allow(dead_code, unused, unused_attributes)]
use std::io::{Error, ErrorKind};

use super::car::Car;
use super::coordinates::Coordinate2D as Coordinate;
use ansi_term::{ANSIGenericString, Color, Style};
use Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TLight {
    Horizontal,
    Vertical,
    None,
}

impl std::fmt::Display for TLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pass = Style::new().on(Color::Green).fg(Color::Black);
        let wait = Style::new().on(Color::Yellow).fg(Color::Black);
        match self {
            &TLight::Horizontal => write!(f, "{}", pass.paint("↔")),
            &TLight::Vertical => write!(f, "{}", pass.paint("↕")),
            &TLight::None => write!(f, "{}", wait.paint("·")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Position {
    Wall,
    Path,
    Road,
    PCrossing(TLight),
    TrafficLight(TLight),
}

impl<T> From<T> for Position
where
    T: AsRef<str>,
{
    fn from(text: T) -> Self {
        let num: usize = text.as_ref().parse().unwrap_or(0);
        match text.as_ref() {
            "R" => Position::Road,
            "r" => Position::Road,
            "T" => Position::TrafficLight(TLight::Horizontal),
            "t" => Position::TrafficLight(TLight::Horizontal),
            "C" => Position::PCrossing(TLight::Horizontal),
            "c" => Position::PCrossing(TLight::Horizontal),
            "P" => Position::Path,
            "p" => Position::Path,
            _ => match num {
                0 => Position::Path,
                1 => Position::Road,
                2 => Position::Wall,
                3 => Position::TrafficLight(TLight::Horizontal),
                4 => Position::PCrossing(TLight::Horizontal),
                _ => Position::Wall,
            },
        }
    }
}

#[derive(Debug)]
pub struct Game {
    map: Vec<Vec<Position>>,
    boundary: Coordinate,
    cars: Vec<Car>,
}

impl Game {
    pub fn cars(&self) -> &Vec<Car> {
        &self.cars
    }

    pub fn map(&self) -> &Vec<Vec<Position>> {
        &self.map
    }

    pub fn boundary(&self) -> Coordinate {
        self.boundary
    }

    pub fn new(map: Vec<Vec<Position>>, cars: Vec<Car>) -> Result<Self, Error> {
        if map.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "Not gut"));
        }
        if let Some(mut max) = map.iter().map(|line| line.len()).max() {
            max -= 1;
            let boundary = Coordinate {
                x: max,
                y: map.len() - 1,
            };
            if map.iter().all(|line| line.len() == max + 1) {
                return Ok(Self {
                    map,
                    cars,
                    boundary,
                });
            }
            return Err(Error::new(ErrorKind::Other, "Not gut"));
        }
        Err(Error::new(ErrorKind::Other, "Not gut"))
    }

    fn move_one(car: &mut Car, boundary: &Coordinate) {
        car.forward(boundary);
    }

    pub fn move_all(&mut self) {
        for car in &mut self.cars {
            Self::move_one(car, &self.boundary);
        }
    }

    fn get_road_or_car(&self, position: &Coordinate) -> ANSIGenericString<str> {
        for car in self.cars() {
            if car.position() == position {
                return car.to_colored_string();
            }
        }
        Style::new()
            .on(Color::RGB(115, 115, 115))
            .fg(Color::Black)
            .paint(" ")
    }

    fn get_path_or_person(&self, position: &Coordinate) -> ANSIGenericString<str> {
        Style::new()
            .on(Color::RGB(8, 51, 6))
            .fg(Color::Black)
            .paint(" ")
    }

    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }

    pub fn position(&self, position: &Coordinate) -> Option<Position> {
        if position.x > self.boundary.x || position.y > self.boundary.y {
            return None;
        }
        Some(self.map[position.y][position.x])
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style_wall = Style::new().on(Color::Black).fg(Color::Black);
        let mut x: usize = 0;
        let mut y: usize = 0;
        for line in &self.map {
            for tile in line {
                match tile {
                    Position::TrafficLight(x) => {
                        write!(f, "{}", style_wall.paint(format!("{}", x)))
                    }
                    Position::PCrossing(x) => write!(f, "{}", style_wall.paint(format!("{}", x))),
                    Position::Wall => write!(f, "{}", style_wall.paint(" ")),
                    Position::Road => {
                        write!(f, "{}", self.get_road_or_car(&Coordinate { x, y }))
                    }
                    Position::Path => {
                        write!(f, "{}", self.get_path_or_person(&Coordinate { x, y }))
                    }
                }?;
                y += 1;
            }
            x += 1;
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
