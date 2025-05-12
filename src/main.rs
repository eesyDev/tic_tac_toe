use iced::{
    Element, Sandbox, Settings, alignment
};

use iced::widget::{button, Button, Column, Container};

pub fn main() {
    println!("Hello, world!");
}

#[derive(Default, Clone)]
struct TicTacToeApp {
    game_field: [[Option<Player>; 3]; 3],
    current_player: Option<Player>,
    game_state: Option<GameState>
}
#[derive(Debug, Clone)]
enum Player {
    XPlayer,
    OPlayer
}
#[derive(Debug, Clone)]
enum GameState {
    InProgress,
    Draw,
    Win(Player)
}