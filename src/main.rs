use snake::{Direction, Game};

fn main() {
    println!("{}", core::mem::size_of::<Game>());
}

trait SnakeStrategy {
    fn decide_move(&self, game: Game) -> Action;
}

enum Action {
    Move(Direction),
    GiveUp,
}
