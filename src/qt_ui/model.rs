use crate::game;
use crate::game::{board, domino};
use crate::log::{info, warn};
use qmetaobject::*;
use std::collections::HashSet;

#[derive(QObject, Default)]
pub struct Domino {
    base: qt_base_class!(trait QObject),
    game_id: qt_property!(domino::Id; CONST),
    board_position: qt_property!(QPointF; CONST),
    head_value: qt_property!(domino::Value; CONST),
    tail_value: qt_property!(domino::Value; CONST),
    horizontal: qt_property!(bool; CONST),
    broken: qt_signal!(),
}

impl Domino {
    fn new(id: domino::Id, domino: game::domino::Domino) -> DominoBox {
        let created = Self {
            base: Default::default(),
            game_id: id,
            board_position: QPointF {
                x: domino.position.x as f64,
                y: domino.position.y as f64,
            },
            head_value: domino.values.head,
            tail_value: domino.values.tail,
            horizontal: domino.orientation == domino::Orientation::Horizontal,
            broken: Default::default(),
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
    score: qt_property!(game::Score; READ score NOTIFY score_changed),
    finished: qt_property!(bool; READ is_finished NOTIFY finished_changed),

    new_game: qt_method!(fn(&self, board_description: String)),
    domino_hit: qt_method!(fn(&self, id: domino::Id)),

    game_changed: qt_signal!(),
    score_changed: qt_signal!(),
    finished_changed: qt_signal!(),

    game: Option<game::Game>,
    m_dominoes: QObjectBox<SimpleListModel<DominoBox>>,
}

impl Game {
    fn dominoes(&mut self) -> QPointer<SimpleListModel<DominoBox>> {
        self.m_dominoes.pinned().borrow().into()
    }

    fn score(&self) -> game::Score {
        self.game
            .as_ref()
            .map(game::Game::score)
            .unwrap_or_default()
    }

    fn new_game(&mut self, board_description: String) {
        info!("Creating new game from:\n{}", board_description);
        let board = board::generator::generate_from_string(&board_description);
        let game = game::Game::new(board);
        let dominoes: SimpleListModel<DominoBox> = game
            .dominoes()
            .iter()
            .map(|(id, domino)| Domino::new(*id, *domino))
            .collect();
        self.m_dominoes = Default::default();
        self.game_changed();
        self.m_dominoes = QObjectBox::new(dominoes);
        self.m_dominoes.pinned().get_or_create_cpp_object();
        self.board_width = game.board().width();
        self.board_height = game.board().height();
        self.game = Some(game);
        self.game_changed();
        self.score_changed();
        self.finished_changed();
    }

    fn domino_hit(&mut self, id: domino::Id) {
        if let Some(game) = &mut self.game {
            let result = game.hit_domino(id);
            let is_finished = game.is_finished();
            self.remove_dominoes(result);
            self.score_changed();
            if is_finished {
                self.finished_changed();
            }
        }
    }

    fn remove_dominoes(&mut self, hit_result: Vec<game::DominoRemoved>) {
        let removed_set: HashSet<domino::Id> = hit_result.iter().map(|d| d.id).collect();
        let mut checked_index = 0;
        let dominoes = self.m_dominoes.pinned();
        let mut dominoes = dominoes.borrow_mut();
        while checked_index < dominoes.row_count() as usize {
            let domino = dominoes[checked_index].0.pinned();
            let domino = domino.borrow();
            if removed_set.contains(&domino.game_id) {
                domino.broken();
                dominoes.remove(checked_index)
            } else {
                checked_index += 1
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.game.as_ref().map_or(true, |game| game.is_finished())
    }
}
