use crate::game;
use crate::game::{board, domino};
use qmetaobject::*;
use std::borrow::Borrow;

#[derive(QObject, Default)]
struct Domino {
    base: qt_base_class!(trait QObject),
    board_position: qt_property!(QPointF; CONST),
    head_value: qt_property!(domino::Value; CONST),
    tail_value: qt_property!(domino::Value; CONST),
}

impl Domino {
    fn new(domino: game::domino::Domino) -> DominoBox {
        let created = Self {
            base: Default::default(),
            board_position: QPointF {
                x: domino.position.x as f64,
                y: domino.position.y as f64,
            },
            head_value: domino.values.head,
            tail_value: domino.values.tail,
        };
        let boxed = QObjectBox::new(created);
        boxed.pinned().get_or_create_cpp_object();
        DominoBox(boxed)
    }
}

#[derive(Default)]
struct DominoBox(QObjectBox<Domino>);

impl SimpleListItem for DominoBox {
    fn get(&self, _role: i32) -> QVariant {
        self.0.pinned().into()
    }

    fn names() -> Vec<QByteArray> {
        vec!["domino".into()]
    }
}

#[derive(QObject, Default)]
pub struct Game {
    base: qt_base_class!(trait QObject),
    board_width: qt_property!(board::Coord; NOTIFY game_changed),
    board_height: qt_property!(board::Coord; NOTIFY game_changed),
    dominoes: qt_property!(QPointer<SimpleListModel<DominoBox>>; READ dominoes NOTIFY game_changed),
    new_game: qt_method!(fn(&self, board_description: String)),
    game_changed: qt_signal!(),
    game: Option<game::Game>,
    m_dominoes: QObjectBox<SimpleListModel<DominoBox>>,
}

impl Game {
    fn new_game(&mut self, board_description: String) {
        let board = board::generator::generate_from_string(&board_description);
        let game = game::Game::new(board);
        let dominoes: SimpleListModel<DominoBox> = game
            .dominoes()
            .iter()
            .map(|(_id, domino)| Domino::new(*domino))
            .collect();
        crate::log::warn!("{}", dominoes.iter().count());
        self.m_dominoes = QObjectBox::new(dominoes);
        self.m_dominoes.pinned().get_or_create_cpp_object();
        self.board_width = game.board().width();
        self.board_height = game.board().height();
        self.game = Some(game);
        self.game_changed();
    }

    fn dominoes(&mut self) -> QPointer<SimpleListModel<DominoBox>> {
        self.m_dominoes.pinned().borrow().into()
    }
}
