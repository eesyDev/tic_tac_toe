#[derive(Debug, Clone)]
pub enum Player {
    XPlayer,
    OPlayer
}
#[derive(Debug, Clone)]
pub enum GameState {
    InProgress,
    Draw,
    Win(Player)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    X,
    O
}

