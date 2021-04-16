mod game;

use game::board;

const LEVEL: &str = r"
----------
|--------|
||------||
|||----|||
||||--||||
||||--||||
|||----|||
||------||
|--------|
";

fn main() {
    let _board = board::generator::generate_from_string(LEVEL);
}
