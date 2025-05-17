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
    CellPressed(usize, usize),
    XPlayerWon,
    OPlayerWon,
    Draw
}

impl Sandbox for TicTacToeApp {
    type Message = Message;
    fn new() -> Self {
        Self {
            current_player: Some(Player::XPlayer),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Tic — Tac — Toe")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PlayerMove(player) => {
                println!("Player x or o");
            }
            Message::CellPressed(row, col) => {
                if self.game_field[row][col].is_none() {
                    self.game_field[row][col] = Some(
                        match self.current_player {
                            Some(Player::XPlayer) => Cell::X,
                            Some(Player::OPlayer) => Cell::O,
                            None => Cell::X
                        }
                    );

                    self.current_player = match self.current_player {
                        Some(Player::XPlayer) => Some(Player::OPlayer),
                        Some(Player::OPlayer) => Some(Player::XPlayer),
                        None => Some(Player::XPlayer)
                    }
                }
            }
            Message::XPlayerWon => {
                println!("Player x have won");
            }
            Message::OPlayerWon => {
                println!("Player 0 have won");
            }
            Message::Draw => {
                println!("Draw");
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let col = Column::with_children(
            self.game_field.iter().enumerate().map(|(row_idx, row)| {
                Row::with_children(row.iter().enumerate().map(|(col_idx, cell)| {
                    let text = match cell {
                        Some(Cell::X) => "X",
                        Some(Cell::O) => "O",
                        Some(Cell::Empty) => " ",
                        None => " "
                    };
                    Button::new(Text::new(text))
                        .width(Length::Fill)
                        .padding(20)
                        .on_press(Message::CellPressed(row_idx, col_idx))
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