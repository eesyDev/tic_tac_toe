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

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    X,
    O
}

