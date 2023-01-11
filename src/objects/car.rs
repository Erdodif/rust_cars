#[allow(dead_code,unused,unused_attributes)]
use ansi_term::{Color, Style, ANSIGenericString};

use super::coordinates::Coordinate2D as Coordinate;
use super::directions::Direction2D as Direction;
use super::map::Position;

#[derive(Debug, Hash)]
pub struct Car {
    position: Coordinate,
    speed: u8,
    top_speed: u8,
    direction: Direction,
}

impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_colored_string())
    }
}


impl From<Coordinate> for Car
{
    fn from(pos: Coordinate) -> Self {
        Car { position: pos, speed: 0, top_speed: 8, direction: Direction::Stopped }
    }
}

impl Car {
    pub fn position(&self) -> &Coordinate {
        &self.position
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn top_speed(&self) -> u8 {
        self.top_speed
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn forward(&mut self, boundary: &Coordinate) {
        self.position = self.position.next_pos(&boundary, &self.direction);
    }

    pub fn turn(&mut self, new_direction: &Direction) {
        if new_direction == &self.direction {
            return;
        }
        if new_direction == &Direction::Stopped {
            self.speed = 0;
        }
        self.direction = *new_direction;
    }

    pub fn new_basic(x: usize, y: usize) -> Car {
        Car {
            position: Coordinate::new(x, y),
            speed: 0,
            top_speed: 8,
            direction: Direction::Stopped,
        }
    }

    pub fn new(x: usize, y: usize, top_speed: u8) -> Car {
        Car {
            position: Coordinate::new(x, y),
            speed: 0,
            top_speed,
            direction: Direction::Stopped,
        }
    }

    pub fn to_colored_string(&self) -> ANSIGenericString<str>{
        Style::new().on(Color::Blue).fg(Color::Black).paint(format!("{}", self.direction))
    }
}
