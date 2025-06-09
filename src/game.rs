use rand;
use std::collections::BTreeMap;

use iced::{ Center, Element };
use iced::widget::{
    button, column, row, text, text_input, center
};

use crate::player::{Role, Player};

#[derive(Debug, Clone)]
pub enum GameMessage {
    Revealing,
    Next,
    Confirm(bool),
    Eliminate(usize),
    WordEdit(String),
    WordGuess,
    Done,
}

#[derive(Debug)]
enum GameState {
    REVEAL = 0,
    POLL,
    GUESS,
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    players: Vec<Player>,
    words: BTreeMap<Role, String>,

    index: usize,
    show: bool,

    first: usize,
    over: bool,
    guess: String,
    winner: Role,
}

impl Default for Game {
    fn default() -> Game {
        let mut names: Vec<String> = Vec::new();
        names.push("Player1".into());
        names.push("Player2".into());
        names.push("Player3".into());

        Game::new(names, 2, 1, false)
    }
}

impl Game {
    fn generate_words() -> BTreeMap<Role, String> {
        let mut words: BTreeMap<Role, String> = BTreeMap::new();
        words.insert(Role::CITIZEN, "Messi".into());
        words.insert(Role::UNDERCOVER, "Ronaldo".into());
        words
    }

    pub fn new(names: Vec<String>, mut citizen: usize, mut undercover: usize, mut white: bool) -> Game {
        if names.len() != citizen + undercover + (white as usize) {
            panic!("There's a different amount of player and roles to attribute");
        }

        let mut players: Vec<Player> = Vec::new();
        let mut i = 0;

        while citizen > 0 || undercover > 0 || white {
            let role: usize = rand::random_range(0..=2);

            if role == (Role::CITIZEN as usize) && citizen > 0 {
                players.push(Player::new(names[i].clone(), Role::CITIZEN));
                citizen -= 1;
                i += 1;
            }

            if role == (Role::UNDERCOVER as usize) && undercover > 0 {
                players.push(Player::new(names[i].clone(), Role::UNDERCOVER));
                undercover -= 1;
                i += 1;
            }

            if role == (Role::WHITE as usize) && white {
                players.push(Player::new(names[i].clone(), Role::WHITE));
                white = false;
                i += 1;
            }
        }

        let mut first = rand::random_range(0..names.len());
        while *players[first].role() == Role::WHITE {
            first = rand::random_range(0..names.len());
        }

        Game {
            state: GameState::REVEAL,
            players: players,
            words: Self::generate_words(),
            index: 0,
            show: false,
            first: first,
            over: false,
            guess: String::from(""),
            winner: Role::CITIZEN,
        }
    }

    pub fn over(&self) -> &bool { &self.over }
    pub fn winner(&self) -> &Role { &self.winner }
    fn is_game_over(&mut self) {
        if self.guess.to_lowercase() == self.words[&Role::CITIZEN].to_lowercase() {
            self.over = true;
            self.winner = Role::WHITE;
        }

        let mut undercover_alive = false;
        let mut count_alive = 0;
        for player in &self.players {
            if *player.alive() {
                count_alive += 1;
                if *player.role() == Role::UNDERCOVER { undercover_alive = true; }
            }
        }

        if !undercover_alive && !self.guess.is_empty() {
            self.over = true;
            self.winner = Role::CITIZEN;
        }
        if undercover_alive && count_alive < 3 && !self.guess.is_empty() {
            self.over = true;
            self.winner = Role::UNDERCOVER;
        }
        if count_alive < 3 && self.guess.is_empty() {
            self.over = true;
            self.winner = Role::WHITE;
        }
    }

    pub fn players_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for player in &self.players { names.push(player.name().clone()); }
        names
    }

    pub fn update(&mut self, message: GameMessage) {
        match message {
            GameMessage::Revealing => { self.show = true; }

            GameMessage::Next => {
                self.index += 1;
                self.show = false;

                if self.index > self.players.len() - 1 {
                    self.state = GameState::POLL;
                }
            }

            GameMessage::Confirm(yes) => {
                if yes {
                    self.players[self.index].set_alive(false);
                    if *self.players[self.index].role() == Role::WHITE {
                        self.state = GameState::GUESS;
                    }
                    self.is_game_over();

                    self.first = rand::random_range(0..self.players.len());
                    let mut player = &self.players[self.first];
                    while !*player.alive() && *player.role() == Role::WHITE {
                        self.first = rand::random_range(0..self.players.len());
                        player = &self.players[self.first];
                    }
                }
                self.show = false;
            }

            GameMessage::Eliminate(index) => {
                self.index = index;
                self.show = true;
            }

            GameMessage::WordEdit(word) => { self.guess = word; }
            GameMessage::WordGuess => {
                self.is_game_over();
                self.state = GameState::POLL;
            }

            GameMessage::Done => {}
        }
    }

    pub fn view(&self) -> Element<GameMessage> {
        let PADDING_SIZE = 20;
        let TEXT_SIZE = 25;
        let mut display = column![];

        if *self.over() {
            let mut msg = String::from("");
            match self.winner() {
                Role::WHITE => { msg = self.winner().to_string() + &String::from(" a gagné!"); }
                _ => { msg = String::from("Les ") + &self.winner().to_string() + &String::from("s ont gagné!"); }
            }
            display = display.push(text(msg).size(TEXT_SIZE));
            display = display.push(text(String::from("Le mot des CIVILs: ") + &self.words[&Role::CITIZEN].to_string()).size(TEXT_SIZE/2));
            display = display.push(text(String::from("Le mot des UNDERCOVERs: ") + &self.words[&Role::UNDERCOVER].to_string()).size(TEXT_SIZE/2));
            display = display.push(button("Menu").on_press(GameMessage::Done));
            display = display.spacing(PADDING_SIZE);
        } else {
            match &self.state {
                GameState::REVEAL => {
                    let player = &self.players[self.index];
                    display = display.push(text(player.name()).size(TEXT_SIZE));

                    if self.show {
                        if *player.role() == Role::WHITE {
                            display = display.push(text("Tu es Mr. White"));
                        } else {
                            display = display.push(text(String::from("Ton mot est: ") + &self.words[player.role()].clone()).size(TEXT_SIZE - 5));
                        }
                        display = display.push(button("Suivant").on_press(GameMessage::Next));
                    } else {
                        display = display.push(button("Révéler").on_press(GameMessage::Revealing));
                    }
                    display = display.spacing(PADDING_SIZE);
                }

                GameState::POLL => {
                    if self.show {
                        let name = self.players[self.index].name();
                        let question = String::from("Exclure ") + name + &String::from("?");
                        display = display.push(text(question).size(TEXT_SIZE));
                        display = display.push(row![
                            button("Non").on_press(GameMessage::Confirm(false)),
                            button("Oui").on_press(GameMessage::Confirm(true))
                        ].spacing(PADDING_SIZE));
                    } else {
                        let name = self.players[self.first].name();
                        display = display.push(row![text(name.clone() + &String::from(" commence")).size(TEXT_SIZE)].padding(PADDING_SIZE));

                        for i in 0..self.players.len() {
                            let player = &self.players[i];
                            let mut player_row = row![];
                            player_row = player_row.push(text(self.players[i].name()).size(TEXT_SIZE));
                            if !*player.alive() {
                                player_row = player_row.push(text(player.role().to_string()));
                            } else {
                                player_row = player_row.push(button("Vote").on_press(GameMessage::Eliminate(i)));
                            }

                            display = display.push(player_row.spacing(PADDING_SIZE).align_y(Center));
                        }
                    }
                }

                GameState::GUESS => {
                    display = display.push(column![
                        text("Devine le mot des CIVILs:"),
                        text_input("", &self.guess).on_input(GameMessage::WordEdit).on_submit(GameMessage::WordGuess),
                        button("Ok").on_press(GameMessage::WordGuess)
                    ].align_x(Center));
                }
            }
        }

        center(display.align_x(Center)).into()
    }
}
