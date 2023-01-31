use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration;

struct CleanUp; // struct for cleaning up incase program panics and cannot exit raw mode

// function to drop raw mode in event of panic
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Couldn't disable raw mode.")
    }
}

fn main() -> crossterm::Result<()> { // this means main will return a Result {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?; // changed to shorthand for expect (? can only be used in a method that returns a Result or Option)

    loop { // looping keeps the program alive and reading lines
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? { // checks if Event returned is a Key -- if program fails to read the line, prints msg
                match event { // attempts to match event (input) to corresponding enum in crossterm::Event
                    KeyEvent {
                        code: KeyCode::Char('q'), 
                        modifiers: event::KeyModifiers::NONE, // modifiers ex. ctrl or alt
                    } => break, // check is Key is 'q', if it is then break out of program
                    _ => {
                        //todo
                    }
                }
                println!("{:?}\r", event);
            };
        } else {
            println!("No input yet.\r");
        }
    }
    Ok(())

}