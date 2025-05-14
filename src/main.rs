use std::io as og_io;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

mod game;
mod combat;
mod io;
mod ui;

//use game::{Action,GameState,Player,Room}
use ui::print_message;



fn main() {
    println!("Hello, young padavan! I am Jedi Master, Creator Alias: Daniel");
    print_message("Welcome to Quest for the Lost Code!", ui::Color::Cyan);
    print_message("You're a Jedi seeking the legendary Bug-free compiler force", ui::Color::Cyan);
    print_message("What's your name young padavan?", ui::Color::Yellow);

    let mut player_name=String::new();


}

