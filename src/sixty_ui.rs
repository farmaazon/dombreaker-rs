mod model;

use crate::game::{domino, Game};
use std::cell::RefCell;
use std::rc::Rc;

sixtyfps::include_modules!();

const LEVEL: &str = "--------\n\
                    |------|\n\
                    ||----||\n\
                    |||--|||\n\
                    |||--|||\n\
                    ||----||\n\
                    |------|";

struct Application {
    game: Game,
    handler: model::Handler,
}

impl Application {
    fn new(main_window: &Main) -> Rc<RefCell<Self>> {
        let game = Game::new_generated(LEVEL);
        let handler = model::Handler::initialize(&main_window, &game);
        let this = Self { game, handler };
        Rc::new(RefCell::new(this))
    }

    fn on_restart_handler(this: Rc<RefCell<Self>>) -> impl Fn() {
        move || {
            let mut borrow = this.borrow_mut();
            let self_ref = &mut *borrow;
            self_ref.game = Game::new_generated(LEVEL);
            self_ref.handler.reinitialize(&self_ref.game);
        }
    }

    fn on_domino_clicked(this: Rc<RefCell<Self>>) -> impl Fn(i32) {
        move |id| {
            let mut self_ref = this.borrow_mut();
            let result = self_ref.game.hit_domino(id as domino::Id);
            self_ref.handler.update(&self_ref.game, &result);
        }
    }
}

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

pub fn main() {
    let main_window = Main::new();
    let application = Application::new(&main_window);
    let model = main_window.global::<GameModel>();
    model.on_restart(Application::on_restart_handler(application.clone()));
    model.on_domino_clicked(Application::on_domino_clicked(application));

    let info = main_window.global::<DominoInfo>();
    info.on_is_dot_visible(is_dot_visible);

    main_window.run();
}
