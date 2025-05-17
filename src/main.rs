use iced::{
    Element, Sandbox, Settings, Length, alignment
};
use iced::widget::{button, Button, Column, Container, Text, Row};
mod types;
use types::{Player, GameState, Cell};

pub fn main() {
    println!("Hello, world!");
    TicTacToeApp::run(Settings::default());
}

#[derive(Default, Clone)]
struct TicTacToeApp {
    game_field: [[Option<Cell>; 3]; 3],
    current_player: Option<Player>,
    game_state: Option<GameState>
}

// println!("{:?}", TicTacToeApp);
#[derive(Debug, Clone)]
enum Message {
    PlayerMove(Player),
    // CellPressed(row, col),
    XPlayerWon,
    OPlayerWon,
    Draw
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

    fn view(&self) -> Element<Message> {

        let col = Column::with_children(
            self.game_field.iter().map(|row| {
                Row::with_children(row.iter().map(|cell| {
                    Button::new(Text::new(" "))
                        .width(Length::Fill)
                        .padding(20)
                        .into()
                    })
                    .collect()
                )
                .spacing(10)
                .into()
            })
            .collect()
        ).spacing(10);
        Container::new(
            Column::new()
                .spacing(10)
                .push(col)
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    }
}