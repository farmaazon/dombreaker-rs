use crate::board::Position;
use std::fmt;

pub type Id = u8;
pub type Value = u8;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Values {
    pub head: Value,
    pub tail: Value,
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}|{}]", self.head, self.tail)
    }
}

impl From<(Value, Value)> for Values {
    fn from((head, tail): (Value, Value)) -> Self {
        Self { head, tail }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Domino {
    pub values: Values,
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
        write!(f, "{}", self.values)
    }
}
