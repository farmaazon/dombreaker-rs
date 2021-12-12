use crate::game::domino::{Domino, Id, Orientation};
use crate::game::{DominoRemoved, Game};
use crate::sixty_ui::{DominoModel, GameModel, Main, Position};
use sixtyfps::{ComponentHandle, Model, ModelHandle, VecModel};
use std::collections::HashSet;
use std::rc::Rc;

impl DominoModel {
    fn from_domino(id: Id, domino: &Domino) -> Self {
        Self {
            game_id: id as i32,
            board_position: Position {
                x: domino.position.x as i32,
                y: domino.position.y as i32,
            },
            head_value: domino.values.head as i32,
            tail_value: domino.values.tail as i32,
            horizontal: domino.orientation == Orientation::Horizontal,
            disappearing: false,
        }
    }
}

#[derive(Clone)]
pub struct Dominoes {
    dominoes: Rc<VecModel<DominoModel>>,
    broken: Rc<VecModel<DominoModel>>,
}

impl Dominoes {
    pub fn new(game: &Game) -> Self {
        let dominoes: Vec<DominoModel> = game
            .dominoes()
            .iter()
            .map(|(id, domino)| DominoModel::from_domino(*id, domino))
            .collect();
        Self {
            dominoes: Rc::new(VecModel::from(dominoes)),
            broken: Rc::new(VecModel::from(vec![])),
        }
    }

    pub fn new_in_game_model(game: &Game, model: &GameModel) -> Self {
        let this = Self::new(game);
        model.set_dominoes(ModelHandle::new(this.dominoes.clone()));
        model.set_broken_dominoes(ModelHandle::new(this.broken.clone()));
        this
    }

    fn update(&self, removed: &[DominoRemoved]) {
        let removed_ids: HashSet<Id> = removed.iter().map(|d| d.id).collect();
        let mut i = 0;
        while i < self.dominoes.row_count() {
            let domino = self.dominoes.row_data(i);
            if removed_ids.contains(&(domino.game_id as Id)) {
                self.dominoes.remove(i);
                self.add_broken_domino(domino);
            } else {
                i += 1;
            }
        }
    }

    fn add_broken_domino(&self, domino: DominoModel) {
        let broken = self.broken.clone();
        let broken_index = broken.row_count();
        broken.push(domino);
        sixtyfps::Timer::single_shot(std::time::Duration::from_millis(300), move || {
            let mut domino = broken.row_data(broken_index);
            domino.disappearing = true;
            broken.set_row_data(broken_index, domino);
        });
    }
}

#[derive(Clone)]
pub struct Handler {
    main: sixtyfps::Weak<Main>,
    dominoes: Dominoes,
}

impl Handler {
    pub fn initialize(main: &Main, game: &Game) -> Self {
        let game_model = main.global::<GameModel>();
        game_model.set_board_width(game.board().width() as i32);
        game_model.set_board_height(game.board().height() as i32);
        game_model.set_score(game.score() as i32);
        game_model.set_finished(game.is_finished());
        let dominoes = Dominoes::new_in_game_model(game, &game_model);
        Self {
            main: main.as_weak(),
            dominoes,
        }
    }

    pub fn update(&self, game: &Game, removed_dominoes: &[DominoRemoved]) {
        let main = self.main.upgrade().unwrap();
        let game_model = main.global::<GameModel>();
        let score_gained = removed_dominoes
            .iter()
            .map(|d| d.score_awarded as i32)
            .sum::<i32>();
        game_model.set_score(game_model.get_score() + score_gained);
        game_model.set_finished(game.is_finished());
        self.dominoes.update(removed_dominoes);
    }
}
