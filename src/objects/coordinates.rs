#[allow(dead_code,unused,unused_attributes)]
use super::directions::Direction2D as Direction;

#[derive(Debug, PartialEq, Eq, Clone, Copy,Hash)]
pub struct Coordinate2D {
    pub x: usize,
    pub y: usize,
}

impl std::ops::Sub for Coordinate2D {
    type Output = Self;

    fn sub(self, rhs: Coordinate2D) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Coordinate2D {
    type Output = Self;

    fn add(self, rhs: Coordinate2D) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<Self> for Coordinate2D {
    type Output = usize;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl std::ops::Mul<usize> for Coordinate2D {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Coordinate2D> for usize {
    type Output = Coordinate2D;
    fn mul(self, rhs: Coordinate2D) -> Self::Output {
        Coordinate2D {
            x: rhs.x * self,
            y: rhs.x * self,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Coordinate2D {
    pub fn new(x: usize, y: usize) -> Coordinate2D {
        Coordinate2D { x, y }
    }

    fn ahead(position: usize, boundary: usize, forward: bool) -> usize {
        if position == boundary && forward {
            return 0;
        }
        if position == 0 && !forward {
            return boundary;
        }
        if forward {
            return position + 1;
        }
        position - 1
    }

    pub fn next_pos(&self, boundary: &Coordinate2D, direction: &Direction) -> Coordinate2D {
        if direction.is_horizontal() {
            return Coordinate2D {
                x: Coordinate2D::ahead(self.x, boundary.x, direction == &Direction::Right),
                y: self.y,
            };
        } else {
            return Coordinate2D {
                x: self.x,
                y: Coordinate2D::ahead(self.y, boundary.y, direction == &Direction::Down),
            };
        }
    }
}
