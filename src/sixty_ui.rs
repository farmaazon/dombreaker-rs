use crate::game::domino::Orientation;
use crate::game::{domino, DominoRemoved, Game};
use sixtyfps::Model;
use sixtyfps::{ModelHandle, VecModel};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

sixtyfps::include_modules!();

fn create_dominoes_model(game: &Game) -> Rc<VecModel<DominoModel>> {
    let dominoes: Vec<DominoModel> = game
        .dominoes()
        .iter()
        .map(|(id, domino)| DominoModel {
            game_id: *id as i32,
            board_position: Position {
                x: domino.position.x as i32,
                y: domino.position.y as i32,
            },
            head_value: domino.values.head as i32,
            tail_value: domino.values.tail as i32,
            horizontal: domino.orientation == Orientation::Horizontal,
        })
        .collect();
    Rc::new(VecModel::from(dominoes))
}

fn initialize_game_model(model: &GameModel, game: &Game) {
    let dominoes = create_dominoes_model(game);
    model.set_board_width(game.board().width() as i32);
    model.set_board_height(game.board().height() as i32);
    model.set_dominoes(ModelHandle::new(dominoes));
    model.set_score(game.score() as i32);
    model.set_finished(game.is_finished());
}

fn update_dominoes(removed_dominoes: &[DominoRemoved], dominoes: &VecModel<DominoModel>) {
    let removed_ids: HashSet<domino::Id> = removed_dominoes.iter().map(|d| d.id).collect();
    let mut i = 0;
    while i < dominoes.row_count() {
        let domino = dominoes.row_data(i);
        if removed_ids.contains(&(domino.game_id as domino::Id)) {
            dominoes.remove(i);
        } else {
            i += 1;
        }
    }
}

fn update_game_model(model: &GameModel, game: &Game, removed_dominoes: &[DominoRemoved]) {
    let score_gained = removed_dominoes
        .iter()
        .map(|d| d.score_awarded as i32)
        .sum::<i32>();
    model.set_score(model.get_score() + score_gained);
    model.set_finished(game.is_finished());
    update_dominoes(
        removed_dominoes,
        model
            .get_dominoes()
            .as_any()
            .downcast_ref::<VecModel<DominoModel>>()
            .unwrap(),
    );
}

const LEVEL: &str = "--------\n\
                    |------|\n\
                    ||----||\n\
                    |||--|||\n\
                    |||--|||\n\
                    ||----||\n\
                    |------|";

pub fn main() {
    let main_window = Main::new();
    let game = Game::new_generated(&LEVEL);
    let model = main_window.global::<GameModel>();
    initialize_game_model(&model, &game);

    let game = RefCell::new(game);
    let weak = main_window.as_weak();
    model.on_domino_clicked(move |id| {
        let main_window = weak.upgrade().unwrap();
        let model = main_window.global::<GameModel>();
        let result = game.borrow_mut().hit_domino(id as domino::Id);
        update_game_model(&model, &*game.borrow(), &result);
    });

    main_window.run();
}
