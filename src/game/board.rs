pub mod generator;

use crate::game::domino;
use crate::game::domino::Domino;

pub type Coord = u8;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: Coord,
    pub y: Coord,
}

impl From<(Coord, Coord)> for Position {
    fn from((x, y): (u8, u8)) -> Self {
        Position { x, y }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Self::N => Self::NE,
            Self::NE => Self::E,
            Self::E => Self::SE,
            Self::SE => Self::S,
            Self::S => Self::SW,
            Self::SW => Self::W,
            Self::W => Self::NW,
            Self::NW => Self::N,
        }
    }

    fn iter(self) -> DirectionIter {
        DirectionIter { next: self }
    }

    fn iter_all() -> impl Iterator<Item = Direction> {
        Self::N.iter().take(8)
    }
}

pub struct DirectionIter {
    next: Direction,
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = self.next.next();
        Some(next)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Head(domino::Id),
    Tail(domino::Id),
}

#[derive(Clone, Debug)]
pub struct Board {
    tiles: Vec<Tile>,
    width: Coord,
    dominoes: std::collections::BTreeMap<domino::Id, Domino>,
    next_domino_id: domino::Id,
}

impl Board {
    pub fn new(width: Coord, height: Coord) -> Self {
        let tiles_len = (width * height) as usize;
        Board {
            tiles: std::iter::repeat(Tile::Empty).take(tiles_len).collect(),
            width,
            dominoes: std::collections::BTreeMap::new(),
            next_domino_id: domino::Id::default(),
        }
    }

    pub fn width(&self) -> Coord {
        self.width
    }

    pub fn height(&self) -> Coord {
        self.tiles.len() as u8 / self.width
    }

    pub fn tile(&self, position: Position) -> Tile {
        self.tiles[self.index_of_tile(position)]
    }

    pub fn domino_values_mut(&mut self, id: domino::Id) -> &mut domino::Values {
        &mut self.dominoes.get_mut(&id).unwrap().values
    }

    pub fn neighbor_of(&self, position: Position, direction: Direction) -> Option<Tile> {
        let Position { x, y } = position;
        let left = position.x.checked_sub(1);
        let right = position.x.checked_add(1).filter(|x| *x < self.width);
        let top = position.y.checked_sub(1);
        let bottom = position.y.checked_add(1).filter(|y| *y < self.height());
        let neighbor_position = match direction {
            Direction::N => Position { x, y: top? },
            Direction::NE => Position { x: right?, y: top? },
            Direction::E => Position { x: right?, y },
            Direction::SE => Position {
                x: right?,
                y: bottom?,
            },
            Direction::S => Position { x, y: bottom? },
            Direction::SW => Position {
                x: left?,
                y: bottom?,
            },
            Direction::W => Position { x: left?, y },
            Direction::NW => Position { x: left?, y: top? },
        };
        Some(self.tile(neighbor_position))
    }

    pub fn all_neighbors_of<'a>(&'a self, position: Position) -> impl Iterator<Item = Tile> + 'a {
        Direction::iter_all().filter_map(move |dir| self.neighbor_of(position, dir))
    }

    pub fn dominoes(&self) -> &std::collections::BTreeMap<domino::Id, Domino> {
        &self.dominoes
    }

    pub fn put_domino(&mut self, domino: Domino) -> u8 {
        let new_id = self.next_domino_id;
        for (tile_pos, new_tile) in &[
            (domino.position, Tile::Head(new_id)),
            (domino.tail_position(), Tile::Tail(new_id)),
        ] {
            let tile = self.tile_mut(*tile_pos);
            if *tile != Tile::Empty {
                panic!(
                    "Put domino {} at already occupied tile {}",
                    domino, tile_pos
                );
            }
            *tile = *new_tile;
        }
        self.dominoes.insert(new_id, domino);
        self.next_domino_id += 1;
        new_id
    }

    pub fn remove_domino(&mut self, domino: u8) -> Domino {
        let removed = self
            .dominoes
            .remove(&domino)
            .expect(&format!("Removing non-existing domino {}", domino));
        for tile_pos in &[removed.position, removed.tail_position()] {
            *self.tile_mut(*tile_pos) = Tile::Empty;
        }
        removed
    }

    fn index_of_tile(&self, position: Position) -> usize {
        let index = (position.y * self.width + position.x) as usize;
        if position.x >= self.width || index >= self.tiles.len() {
            panic!("Tile {} out of bounds", position);
        }
        index
    }

    fn tile_mut(&mut self, position: Position) -> &mut Tile {
        let index = self.index_of_tile(position);
        &mut self.tiles[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    fn assert_empty(board: &Board, pos: impl Into<Position>) {
        assert_eq!(board.tile(pos.into()), Tile::Empty);
    }

    #[test]
    fn construction() {
        let board = Board::new(5, 7);
        assert_eq!(board.width(), 5);
        assert_eq!(board.height(), 7);
        assert_empty(&board, (0, 0));
        assert_empty(&board, (4, 0));
        assert_empty(&board, (0, 6));
        assert_empty(&board, (4, 6));
    }

    #[test]
    fn putting_dominoes() {
        let mut board = Board::new(3, 3);

        let first_domino = Domino {
            values: (1, 2).into(),
            position: (0, 0).into(),
            orientation: domino::Orientation::Vertical,
        };
        let second_domino = Domino {
            values: (2, 6).into(),
            position: (1, 1).into(),
            orientation: domino::Orientation::Horizontal,
        };

        assert_empty(&board, (0, 0));
        assert_empty(&board, (0, 1));
        assert_empty(&board, (1, 1));
        assert_empty(&board, (2, 1));

        let first_id = board.put_domino(first_domino);
        assert_eq!(board.dominoes[&first_id], first_domino);
        assert_eq!(board.tile((0, 0).into()), Tile::Head(first_id));
        assert_eq!(board.tile((0, 1).into()), Tile::Tail(first_id));
        assert_empty(&board, (0, 2));
        assert_empty(&board, (1, 0));
        assert_empty(&board, (1, 1));
        assert_empty(&board, (1, 2));
        assert_empty(&board, (2, 1));
        assert_eq!(
            board.dominoes().iter().collect_vec(),
            vec![(&first_id, &first_domino)]
        );

        let second_id = board.put_domino(second_domino);
        assert_eq!(board.dominoes[&first_id], first_domino);
        assert_eq!(board.dominoes[&second_id], second_domino);
        assert_eq!(board.tile((0, 0).into()), Tile::Head(first_id));
        assert_eq!(board.tile((0, 1).into()), Tile::Tail(first_id));
        assert_eq!(board.tile((1, 1).into()), Tile::Head(second_id));
        assert_eq!(board.tile((2, 1).into()), Tile::Tail(second_id));
        assert_empty(&board, (0, 2));
        assert_empty(&board, (1, 0));
        assert_empty(&board, (1, 2));
        assert_empty(&board, (2, 0));
        assert_empty(&board, (2, 2));

        assert_eq!(
            board.dominoes().iter().collect_vec(),
            vec![(&first_id, &first_domino), (&second_id, &second_domino)]
        );
    }

    #[test]
    #[should_panic]
    fn colliding_dominoes() {
        let mut board = Board::new(3, 3);
        let first_domino = Domino {
            values: (1, 2).into(),
            position: (1, 0).into(),
            orientation: domino::Orientation::Vertical,
        };
        let second_domino = Domino {
            values: (2, 6).into(),
            position: (1, 1).into(),
            orientation: domino::Orientation::Horizontal,
        };
        board.put_domino(first_domino);
        board.put_domino(second_domino);
    }

    #[test]
    #[should_panic]
    fn putting_outside_x_boundaries() {
        let mut board = Board::new(3, 3);
        board.put_domino(Domino {
            values: (1, 2).into(),
            position: (2, 0).into(),
            orientation: domino::Orientation::Horizontal,
        });
    }

    #[test]
    #[should_panic]
    fn putting_outside_y_boundaries() {
        let mut board = Board::new(3, 3);
        board.put_domino(Domino {
            values: (1, 2).into(),
            position: (0, 2).into(),
            orientation: domino::Orientation::Vertical,
        });
    }

    #[test]
    fn removing_dominoes() {
        let mut board = Board::new(3, 3);
        let first_domino = Domino {
            values: (1, 2).into(),
            position: (0, 0).into(),
            orientation: domino::Orientation::Vertical,
        };
        let second_domino = Domino {
            values: (2, 6).into(),
            position: (1, 1).into(),
            orientation: domino::Orientation::Horizontal,
        };
        let first_id = board.put_domino(first_domino);
        let second_id = board.put_domino(second_domino);

        board.remove_domino(first_id);
        assert_empty(&board, (0, 0));
        assert_empty(&board, (0, 1));
        assert_eq!(board.tile((1, 1).into()), Tile::Head(second_id));
        assert_eq!(board.tile((2, 1).into()), Tile::Tail(second_id));
        assert_eq!(
            board.dominoes().iter().collect_vec(),
            vec![(&second_id, &second_domino)]
        );
    }
}
