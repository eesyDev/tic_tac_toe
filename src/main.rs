use iced::{
    Element, Sandbox, Settings, alignment
};

use iced::widget::{button, Button, Column, Container, Text};

pub fn main() {
    println!("Hello, world!");
    TicTacToeApp::run(Settings::default());
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

#[derive(Debug, Clone, Copy)]
enum Message {

}

impl Sandbox for TicTacToeApp {
    type Message = Message;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Tic — Tac — Toe")
    }

    fn update(&mut self, _message: Self::Message) {

    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("Hello Iced").into()
    }
}