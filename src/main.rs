use iced;
use iced::Element;

mod player;
mod game;
mod menu;
use crate::menu::{Menu, MenuMessage};
use crate::game::{Game, GameMessage};

#[derive(Debug)]
enum AppMessage {
    InteractMenu(MenuMessage),
    InteractGame(GameMessage),
}

#[derive(PartialEq, Eq)]
enum AppState {
    StateMenu = 0,
    StateGame,
}

struct Application {
    state: AppState,
    menu: Menu,
    game: Game
}

impl Default for Application {
    fn default() -> Application {
        Application{
            state: AppState::StateMenu,
            menu: Menu::default(),
            game: Game::default(),
        }
    }
}

impl Application {
    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::InteractMenu(menu_msg) => {
                match menu_msg {
                    MenuMessage::Done => {
                        self.state = AppState::StateGame;
                        self.game = Game::new(
                            self.menu.names().clone(),
                            *self.menu.citizen(),
                            *self.menu.undercover(),
                            *self.menu.white()
                        );
                    }

                    _ => {
                        if self.state == AppState::StateMenu { self.menu.update(menu_msg); }
                    }
                }
            }

            AppMessage::InteractGame(game_msg) => {
                match game_msg {
                    GameMessage::Done => {
                        self.state = AppState::StateMenu;
                        self.menu = Menu::new(self.game.players_names());
                    }

                    _ => {
                        if self.state == AppState::StateGame { self.game.update(game_msg); }
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<AppMessage> {
        let mut element;

        match self.state {
            AppState::StateMenu => { element = self.menu.view().map(AppMessage::InteractMenu); }
            AppState::StateGame => { element = self.game.view().map(AppMessage::InteractGame); }
        }

        element
    }
}

fn main() -> iced::Result {
    iced::run("Undercover", Application::update, Application::view)
}
