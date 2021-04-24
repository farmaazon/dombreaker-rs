pub mod board;
pub mod domino;

use std::collections::{BTreeMap, VecDeque};

use crate::game::board::Board;
use domino::Domino;

pub const EXPLOSIVE_VALUE: domino::Value = 0;

pub type Score = i16;

#[derive(Clone, Debug)]
pub struct Game {
    board: Board,
    score: Score,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DominoRemoved {
    pub id: domino::Id,
    pub exploded: bool,
    pub hit_by_explosion: bool,
    pub score_awarded: Score,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Self { board, score: 0 }
    }

    pub fn new_generated(input: &str) -> Self {
        Self::new(board::generator::generate_from_string(input))
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn dominoes(&self) -> &BTreeMap<domino::Id, Domino> {
        self.board.dominoes()
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn is_finished(&self) -> bool {
        self.dominoes().is_empty()
    }

    pub fn hit_domino(&mut self, id: domino::Id) -> Vec<DominoRemoved> {
        let mut dominoes_removed = Vec::new();
        let mut exploded_queue = std::collections::VecDeque::new();

        let hitting_outcome = self.remove_domino(id, false, &mut exploded_queue);
        dominoes_removed.push(hitting_outcome);
        while let Some(exploded) = exploded_queue.pop_front() {
            if self.board.dominoes().contains_key(&exploded) {
                let explosion_outcome = self.remove_domino(exploded, true, &mut exploded_queue);
                dominoes_removed.push(explosion_outcome);
            }
        }
        dominoes_removed
    }

    fn remove_domino(
        &mut self,
        id: domino::Id,
        hit_by_explosion: bool,
        exploded_queue: &mut VecDeque<domino::Id>,
    ) -> DominoRemoved {
        let removed = self.board.remove_domino(id);
        let score_value = (removed.values.head + removed.values.tail) as Score;
        let exploded = self.handle_possible_explosion(removed, exploded_queue);
        let score_awarded = if hit_by_explosion || exploded {
            score_value
        } else if removed.values.head == removed.values.tail {
            -score_value
        } else {
            0
        };
        self.score += score_awarded;
        DominoRemoved {
            id,
            exploded,
            hit_by_explosion,
            score_awarded,
        }
    }

    fn handle_possible_explosion(
        &self,
        removed: Domino,
        exploded_queue: &mut VecDeque<domino::Id>,
    ) -> bool {
        let possible_explosions = &[
            (removed.values.head, removed.position),
            (removed.values.tail, removed.tail_position()),
        ];
        let explosions = possible_explosions
            .iter()
            .filter(|(value, _)| *value == EXPLOSIVE_VALUE)
            .map(|(_, position)| position);
        for explosion in explosions.clone() {
            for tile in self.board.all_neighbors_of(*explosion) {
                match tile {
                    board::Tile::Empty => {}
                    board::Tile::Head(id) | board::Tile::Tail(id) => exploded_queue.push_back(id),
                }
            }
        }
        explosions.count() > 0
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn hitting_normal_domino() {
        let mut board = Board::new(3, 3);
        let domino = board.put_domino(Domino {
            values: (1, 2).into(),
            position: (1, 1).into(),
            orientation: domino::Orientation::Horizontal,
        });
        let mut game = Game::new(board);
        let outcome = game.hit_domino(domino);
        assert_eq!(
            outcome,
            vec![DominoRemoved {
                id: domino,
                exploded: false,
                hit_by_explosion: false,
                score_awarded: 0
            }]
        );
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn hitting_hard_domino() {
        let mut board = Board::new(3, 3);
        let domino = board.put_domino(Domino {
            values: (5, 5).into(),
            position: (1, 1).into(),
            orientation: domino::Orientation::Vertical,
        });
        let mut game = Game::new(board);
        let outcome = game.hit_domino(domino);
        assert_eq!(
            outcome,
            vec![DominoRemoved {
                id: domino,
                exploded: false,
                hit_by_explosion: false,
                score_awarded: -10
            }]
        );
        assert_eq!(game.score(), -10);
    }

    #[test]
    fn hitting_explosive_domino() {
        let mut board = Board::new(3, 3);
        let domino = board.put_domino(Domino {
            values: (5, 0).into(),
            position: (1, 1).into(),
            orientation: domino::Orientation::Vertical,
        });
        let mut game = Game::new(board);
        let outcome = game.hit_domino(domino);
        assert_eq!(
            outcome,
            vec![DominoRemoved {
                id: domino,
                exploded: true,
                hit_by_explosion: false,
                score_awarded: 5
            }]
        );
        assert_eq!(game.score(), 5);
    }

    #[test]
    fn explosion() {
        let mut board = Board::new(3, 6);
        let orientation = domino::Orientation::Vertical;
        let explosive = board.put_domino(Domino {
            values: (5, 0).into(),
            position: (1, 2).into(),
            orientation,
        });

        let not_destroyed = vec![
            Domino {
                values: (0, 1).into(),
                position: (0, 0).into(),
                orientation,
            },
            Domino {
                values: (1, 2).into(),
                position: (1, 0).into(),
                orientation,
            },
            Domino {
                values: (2, 3).into(),
                position: (2, 0).into(),
                orientation,
            },
        ];
        let expected_remaining: BTreeMap<domino::Id, Domino> = not_destroyed
            .iter()
            .map(|domino| (board.put_domino(*domino), *domino))
            .collect();

        let destroyed = vec![
            board.put_domino(Domino {
                values: (3, 4).into(),
                position: (0, 2).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (6, 1).into(),
                position: (2, 2).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (2, 2).into(),
                position: (0, 4).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (3, 3).into(),
                position: (1, 4).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (4, 4).into(),
                position: (2, 4).into(),
                orientation,
            }),
        ];

        let mut game = Game::new(board);

        let expected_outcome = vec![
            DominoRemoved {
                id: explosive,
                exploded: true,
                hit_by_explosion: false,
                score_awarded: 5,
            },
            DominoRemoved {
                id: destroyed[1],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 7,
            },
            DominoRemoved {
                id: destroyed[4],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 8,
            },
            DominoRemoved {
                id: destroyed[3],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 6,
            },
            DominoRemoved {
                id: destroyed[2],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 4,
            },
            DominoRemoved {
                id: destroyed[0],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 7,
            },
        ];
        assert_eq!(game.hit_domino(explosive), expected_outcome);
        assert_eq!(*game.dominoes(), expected_remaining);
        assert_eq!(game.score(), 37)
    }

    #[test]
    fn chained_explosion() {
        let mut board = Board::new(4, 7);
        let orientation = domino::Orientation::Horizontal;
        let hit = board.put_domino(Domino {
            values: (0, 0).into(),
            position: (1, 2).into(),
            orientation,
        });

        let not_destroyed = vec![Domino {
            values: (3, 3).into(),
            position: (1, 6).into(),
            orientation,
        }];
        let expected_remaining: BTreeMap<domino::Id, Domino> = not_destroyed
            .iter()
            .map(|domino| (board.put_domino(*domino), *domino))
            .collect();

        let destroyed = vec![
            board.put_domino(Domino {
                values: (3, 2).into(),
                position: (1, 0).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (3, 0).into(),
                position: (1, 1).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (4, 0).into(),
                position: (1, 3).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (5, 0).into(),
                position: (1, 4).into(),
                orientation,
            }),
            board.put_domino(Domino {
                values: (1, 3).into(),
                position: (1, 5).into(),
                orientation,
            }),
        ];

        let mut game = Game::new(board);

        let expected_outcome = vec![
            DominoRemoved {
                id: hit,
                exploded: true,
                hit_by_explosion: false,
                score_awarded: 0,
            },
            DominoRemoved {
                id: destroyed[1],
                exploded: true,
                hit_by_explosion: true,
                score_awarded: 3,
            },
            DominoRemoved {
                id: destroyed[2],
                exploded: true,
                hit_by_explosion: true,
                score_awarded: 4,
            },
            DominoRemoved {
                id: destroyed[0],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 5,
            },
            DominoRemoved {
                id: destroyed[3],
                exploded: true,
                hit_by_explosion: true,
                score_awarded: 5,
            },
            DominoRemoved {
                id: destroyed[4],
                exploded: false,
                hit_by_explosion: true,
                score_awarded: 4,
            },
        ];
        assert_eq!(game.hit_domino(hit), expected_outcome);
        assert_eq!(*game.dominoes(), expected_remaining);
        assert_eq!(game.score(), 21)
    }
}

#[test]
fn on_edge_explosions_do_not_panic() {
    let mut board = Board::new(4, 4);
    let hit = board.put_domino(Domino {
        values: (0, 0).into(),
        position: (0, 1).into(),
        orientation: domino::Orientation::Vertical,
    });
    board.put_domino(Domino {
        values: (0, 0).into(),
        position: (1, 0).into(),
        orientation: domino::Orientation::Horizontal,
    });
    board.put_domino(Domino {
        values: (0, 0).into(),
        position: (3, 1).into(),
        orientation: domino::Orientation::Vertical,
    });
    board.put_domino(Domino {
        values: (0, 0).into(),
        position: (1, 3).into(),
        orientation: domino::Orientation::Horizontal,
    });

    let mut game = Game::new(board);

    let outcome = game.hit_domino(hit);

    assert_eq!(outcome.len(), 4);
}
