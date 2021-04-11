use crate::board::Position;
use std::fmt;

pub type Id = u8;
pub type Value = u8;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Domino {
    pub head_value: Value,
    pub tail_value: Value,
    pub position: Position,
    pub orientation: Orientation,
}

impl Domino {
    pub fn tail_position(&self) -> Position {
        match self.orientation {
            Orientation::Horizontal => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Orientation::Vertical => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
        }
    }
}

impl fmt::Display for Domino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}|{}]", self.head_value, self.tail_value)
    }
}
