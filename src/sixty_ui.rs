use crate::game::domino::Orientation;
use crate::game::{domino, DominoRemoved, Game};
use sixtyfps::Model;
use sixtyfps::{ModelHandle, VecModel};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

sixtyfps::include_modules!();

fn create_game_model(game: &Game) -> GameModel {
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
    GameModel {
        board_width: game.board().width() as i32,
        board_height: game.board().height() as i32,
        dominoes: ModelHandle::new(Rc::new(VecModel::from(dominoes))),
        score: game.score() as i32,
        finished: game.is_finished(),
    }
}

fn update_game_model(removed_dominoes: &[DominoRemoved], model: &mut GameModel) {
    model.score += removed_dominoes
        .iter()
        .map(|d| d.score_awarded as i32)
        .sum::<i32>();
    let removed_ids: HashSet<domino::Id> = removed_dominoes.iter().map(|d| d.id).collect();
    let dominoes = model
        .dominoes
        .as_any()
        .downcast_ref::<VecModel<DominoModel>>()
        .unwrap();
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

const LEVEL: &str = "--------\n\
                    |------|\n\
                    ||----||\n\
                    |||--|||\n\
                    |||--|||\n\
                    ||----||\n\
                    |------|";

pub fn main() {
    let main_window = Main::new();
    let game = Game::new_generated(&LEVEL.repeat(50));
    let model = create_game_model(&game);
    main_window.set_game(model);

    let game = RefCell::new(game);
    let weak = main_window.as_weak();
    main_window.on_domino_clicked(move |id| {
        let main_window = weak.upgrade().unwrap();
        let result = game.borrow_mut().hit_domino(id as domino::Id);
        let mut model = main_window.get_game();
        update_game_model(&result, &mut model);
        main_window.set_game(model);
    });
    main_window.run();
}
