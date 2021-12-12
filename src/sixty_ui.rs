mod model;

use crate::game::{domino, Game};
use std::cell::RefCell;

sixtyfps::include_modules!();

fn is_dot_visible(
    DotInfo {
        mut row,
        mut col,
        is_horizontal,
        value,
    }: DotInfo,
) -> bool {
    const VERTICAL_THRESHOLDS: [[i32; 3]; 3] = [[4, 8, 2], [6, 0, 6], [2, 8, 4]];
    if is_horizontal {
        let new_col = 2 - row;
        row = col;
        col = new_col;
    }
    if row == 1 && col == 1 {
        value % 2 == 1
    } else {
        value >= VERTICAL_THRESHOLDS[row as usize][col as usize]
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
    let game = Game::new_generated(&LEVEL);
    let model = main_window.global::<GameModel>();
    let info = main_window.global::<DominoInfo>();
    info.on_is_dot_visible(is_dot_visible);

    let handler = model::Handler::initialize(&main_window, &game);
    let game = RefCell::new(game);
    model.on_domino_clicked(move |id| {
        let mut game = game.borrow_mut();
        let result = game.hit_domino(id as domino::Id);
        handler.update(&game, &result);
    });

    main_window.run();
}
