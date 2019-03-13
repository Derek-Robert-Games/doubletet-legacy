use specs::prelude::*;

pub struct MenuCommands {
    pub new_game: bool,
    pub quit: bool,
    pub print_lol: bool
}

impl MenuCommands {
    pub fn has_command(&self) -> bool {
        return self.new_game || self.quit || self.print_lol;
    }
    pub fn clear_all_commands(&mut self) {
        self.new_game = false;
        self.quit = false;
        self.print_lol = false;
    }
}

pub struct MenuNavigation {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool,
    pub escape: bool,
    pub enter: bool
}

impl MenuNavigation {
    pub fn has_movement_action(&self) -> bool {
        return self.move_up || self.move_down || self.move_right || self.move_left;
    }
    pub fn clear_all_actions(&mut self) {
        self.move_up = false;
        self.move_down = false;
        self.move_right = false;
        self.move_left = false;
        self.escape = false;
        self.enter = false;
    }
}

#[derive(Component, Debug)]
pub struct MenuItem {
    // The typical OOP thing to do here would be to have a MenuAction abstract class,
    // with 'action: MenuAction', and you would have three different concrete MenuAction classes,
    // 'quit', 'new_game', etc.
    pub action: String,
    pub title: String,
    pub order: u8,
    pub id: u8
}
