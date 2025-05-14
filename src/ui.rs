use crossterm::style::{Color as CrosstermColor, Print,ResetColor, SetForegroundColor};
use crossterm::{execute,queue};
use std::io::{stdout,Write};

#[derive(Clone,Copy)]
pub enum Color{
    White,
    Red,
    Green,
    Yellow,
    Cyan,
    Magenta,
}

pub fn print_message(message: impl Into<String>,color:Color){
    let crosstem_color=match color{
        Color::White=>CrosstermColor::White,
        Color::Red=>CrosstermColor::Red,
        Color::Green=>CrosstermColor::Green,
        Color::Yellow=>CrosstermColor::Yellow,
        Color::Cyan=>CrosstermColor::Cyan,
        Color::Magenta=>CrosstermColor::Magenta
    };
    
let mut stdout=stdout();
queue!(
    stdout,
    SetForegroundColor(crosstem_color),
    Print(message.into() + "\n"),
    ResetColor
).expect("Failed to queue crossterm commands");

stdout.flush().expect("Failed to flush stdout")
}