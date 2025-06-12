use iced::{ Center, Element };
use iced::widget::{
    Column, column, Row, row,
    button, checkbox,
    text, text_input,
    center
};

#[derive(Debug, Clone)]
pub enum MenuMessage {
    IncrementUndercover,
    DecrementUndercover,
    EnableWhite(bool),
    AddName,
    RemoveName(usize),
    EditIndex(usize),
    EditName(String),
    Done
}

#[derive(Debug)]
pub struct Menu {
    citizen: usize,
    undercover: usize,
    white: bool,

    names: Vec<String>,
    index: usize,
}

impl Default for Menu {
    fn default() -> Menu {
        let mut names = Vec::new();
        names.push("Player1".into());
        names.push("Player2".into());
        names.push("Player3".into());

        Menu {
            citizen: 2,
            undercover: 1,
            white: false,
            names,
            index: 0,
        }
    }
}

impl Menu {
    pub fn new(names: Vec<String>) -> Menu {
        Menu {
            citizen: names.len()-1,
            undercover: 1,
            white: false,
            names,
            index: 0,
        }
    }

    pub fn citizen(&self) -> &usize { &self.citizen }
    pub fn undercover(&self) -> &usize { &self.undercover }
    pub fn white(&self) -> &bool { &self.white }
    pub fn names(&self) -> &Vec<String> { &self.names }

    fn add_undercover(&mut self) {
        if self.citizen > 2 {
            self.undercover += 1;
            self.citizen -= 1;
        }
    }
    fn remove_undercover(&mut self) {
        if self.undercover > 1 {
            self.undercover -= 1;
            self.citizen += 1;
        }
    }

    fn enable_white(&mut self) {
        if self.citizen > 2 {
            self.citizen -= 1;
            self.white = true;
        }
    }
    fn disable_white(&mut self) {
        self.citizen += 1;
        self.white = false;
    }

    fn add_name(&mut self) {
        self.index = self.names.len();
        self.names.push("New Player".into());
        self.citizen += 1;
    }

    fn remove_name(&mut self, i: usize) {
        if self.names.len() > 3 && i < self.names.len() {
            self.names.remove(i);
            if self.citizen < 2 {
                self.undercover -= 1;
            } else {
                self.citizen -= 1;
            }
        }
    }

    fn edit_name(&mut self, name: String) {
        self.names[self.index] = name;
    }

    pub fn update(&mut self, message: MenuMessage) {
        match message {
            MenuMessage::IncrementUndercover => { self.add_undercover(); }
            MenuMessage::DecrementUndercover => { self.remove_undercover(); }

            MenuMessage::EnableWhite(enable) => {
                if enable { self.enable_white() } else { self.disable_white() }
            }

            MenuMessage::AddName => { self.add_name(); }
            MenuMessage::RemoveName(index) => { self.remove_name(index); }

            MenuMessage::EditIndex(index) => { self.index = index; }
            MenuMessage::EditName(name) => { self.edit_name(name); }

            MenuMessage::Done => {}
        }
    }

    pub fn view(&self) -> Element<MenuMessage> {
        let padding_size = 20;
        let text_size = 25;

        let undercover_row: Row<MenuMessage> = row![
            button("-").on_press(MenuMessage::DecrementUndercover),
            text("Undercover: "), text(self.undercover),
            button("+").on_press(MenuMessage::IncrementUndercover)
        ].align_y(Center);

        let mut player_list = column![];
        for i in 0..self.names.len() {
            let mut player_row = row![];

            if i == self.index {
                player_row = player_row.push(text_input("", &self.names[i]).on_input(MenuMessage::EditName).size(text_size));
            } else {
                player_row = player_row.push(text(self.names[i].clone()).size(text_size - 10));
                player_row = player_row.push(button("Edit").on_press(MenuMessage::EditIndex(i)));
            }
            player_row = player_row.push(button("Delete").on_press(MenuMessage::RemoveName(i)));
            player_row = player_row.align_y(Center).spacing(padding_size);

            player_list = player_list.push(player_row);
        }

        let content: Column<MenuMessage> = column![
            text("Young New Undercover").size(text_size*2),
            column![
                text(String::from("Citizen: ") + &self.citizen.to_string()),
                undercover_row,
                checkbox("Mr. White", self.white).on_toggle(MenuMessage::EnableWhite)
            ].align_x(Center).padding(20),
            column![
                player_list,
                button("Add Player").on_press(MenuMessage::AddName),
                button("Ok").on_press(MenuMessage::Done)
            ].align_x(Center).spacing(padding_size/2).padding(padding_size)
        ].align_x(Center).into();

        center(content).padding(padding_size * 2).into()
    }
}
