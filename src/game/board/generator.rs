use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::game::board;
use crate::game::board::{Board, Tile};
use crate::game::domino;
use crate::game::domino::Domino;

#[derive(Copy, Clone, Debug)]
struct DominoValuesGenerator {
    next: domino::Values,
}

impl DominoValuesGenerator {
    fn new() -> Self {
        DominoValuesGenerator {
            next: domino::Values { head: 0, tail: 0 },
        }
    }
}

impl Iterator for DominoValuesGenerator {
    type Item = domino::Values;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.next;
        if self.next.tail >= self.next.head {
            self.next.head += 1;
            self.next.tail = 0;
        } else {
            self.next.tail += 1;
        }
        Some(return_value)
    }
}

struct Generator<'a> {
    input: &'a str,
    board: Board,
}

impl<'a> Generator<'a> {
    fn prepare(input: &'a str) -> Self {
        let lines = input.split('\n');
        let width = lines.clone().map(|l| l.chars().count()).max().unwrap() as board::Coord;
        let height = lines.clone().count() as board::Coord;
        Self {
            input,
            board: Board::new(width, height),
        }
    }

    fn place_dominoes(&mut self) {
        for (row, line) in self.input.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                let position = board::Position {
                    x: col as board::Coord,
                    y: row as board::Coord,
                };
                let possible_domino = match char {
                    '|' => Some(domino::Orientation::Vertical),
                    '-' => Some(domino::Orientation::Horizontal),
                    _ => None,
                };
                if let Some(orientation) = possible_domino {
                    self.fill_tile(position, orientation);
                }
            }
        }
    }

    fn fill_tile(&mut self, position: board::Position, orientation: domino::Orientation) {
        match self.board.tile(position) {
            Tile::Empty => {
                self.board.put_domino(Domino {
                    values: domino::Values::default(),
                    position,
                    orientation,
                });
            }
            Tile::Head(id) | Tile::Tail(id) => {
                if self.board.dominoes()[&id].orientation != orientation {
                    panic!("Inconsistent domino at position {}", position);
                }
            }
        }
    }

    fn assign_values(&mut self) {
        let mut ids = self.board.dominoes().keys().cloned().collect_vec();
        let mut rng = rand::thread_rng();
        ids.shuffle(&mut rng);
        for (id, values) in ids.iter().zip(DominoValuesGenerator::new()) {
            *self.board.domino_values_mut(*id) = if rng.gen_bool(0.5) {
                values.swapped()
            } else {
                values
            }
        }
    }
}

pub fn generate_from_string(string: &str) -> Board {
    let mut generator = Generator::prepare(string);
    generator.place_dominoes();
    generator.assign_values();
    generator.board
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn domino_values_generator() {
        let generated = DominoValuesGenerator::new()
            .take(10)
            .map(|domino::Values { head, tail }| (head, tail))
            .collect_vec();
        assert_eq!(
            generated,
            [
                (0, 0),
                (1, 0),
                (1, 1),
                (2, 0),
                (2, 1),
                (2, 2),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3)
            ]
        )
    }

    #[test]
    fn generate_single_horizontal() {
        let input = "\n--\n";
        let board = generate_from_string(input);
        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 3);
        let expected_domino = Domino {
            values: (0, 0).into(),
            position: (0, 1).into(),
            orientation: domino::Orientation::Horizontal,
        };
        assert_eq!(
            board.dominoes().values().collect_vec(),
            vec![&expected_domino]
        )
    }
    #[test]
    fn generate_single_vertical() {
        let input = " |\n |\n";
        let board = generate_from_string(input);
        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 3);
        let expected_domino = Domino {
            values: (0, 0).into(),
            position: (1, 0).into(),
            orientation: domino::Orientation::Vertical,
        };
        assert_eq!(
            board.dominoes().values().collect_vec(),
            vec![&expected_domino]
        )
    }

    #[test]
    #[should_panic]
    fn panic_on_inconsistency1() {
        let input = "\n\
        --|\n\
        |--\n\
        |--
        ";
        generate_from_string(input);
    }

    #[test]
    #[should_panic]
    fn panic_on_inconsistency2() {
        let input = "\n\
        --|\n\
        ---";
        generate_from_string(input);
    }

    #[test]
    fn several_dominoes() {
        let input = "\n\
        --|--|\n\
        --|--||\n      \
              |";
        let board = generate_from_string(input);
        assert_eq!(board.width(), 7);
        assert_eq!(board.height(), 4);
        println!(
            "[{}]",
            board.dominoes().values().map(ToString::to_string).join(",")
        );

        let mut expected_values: HashSet<domino::Values> =
            [(0, 0), (1, 0), (1, 1), (2, 0), (2, 1), (2, 2), (3, 0)]
                .iter()
                .map(|x| domino::Values::from(*x))
                .collect();
        for domino in board.dominoes().values() {
            assert!(
                expected_values.remove(&domino.values)
                    || expected_values.remove(&domino.values.swapped()),
                "Didn't expected {} domino",
                domino
            );
        }
        assert_eq!(
            expected_values.into_iter().collect_vec(),
            Vec::<domino::Values>::new()
        );
    }
}
