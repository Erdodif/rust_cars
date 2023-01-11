#[derive(Debug, PartialEq, Eq, Clone, Copy,Hash)]
pub enum Direction2D {
    Left,
    Right,
    Up,
    Down,
    Stopped,
}

impl Direction2D {
    pub fn is_horizontal(&self) -> bool {
        self == &Direction2D::Left || self == &Direction2D::Right
    }
}

impl std::fmt::Display for Direction2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Direction2D::Left => write!(f, "{}", '←'),
            &Direction2D::Right => write!(f, "{}", '→'),
            &Direction2D::Up => write!(f, "{}", '↑'),
            &Direction2D::Down => write!(f, "{}", '↓'),
            &Direction2D::Stopped => write!(f, "{}", '·'),
        }
    }
}
