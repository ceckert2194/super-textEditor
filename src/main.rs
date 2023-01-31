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

fn main() {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode().expect("Couldn't enable raw mode.");

    loop { // looping keeps the program alive and reading lines
        if event::poll(Duration::from_millis(500)).expect("Error") {
            if let Event::Key(event) = event::read().expect("Failed to read line.") { // checks if Event returned is a Key -- if program fails to read the line, prints msg
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


    /* old with std lib

    let mut buffer = [0; 1]; // read one byte at a time


    while io::stdin().read(&mut buffer).expect("Failed to read line.") == 1 && buffer != [b'q'] {

        let character = buffer[0] as char;
        // control characters are non-printable characters (ASCII 0-31 + 127)
        if character.is_control() {
            // \r (carriage return) will move the cursor back to the left side of the screen so it doesn't look choppy when typing
            println!("{}\r", character as u8)
        } else {
            println!("{}\r", character)
        }

    }     
    // ctrl-D to tell program it's the end of the file or ctrl-C to terminate
    // the code after the && allows us to enter q to quit -- the prefix b designates a byte literal which is equivalent to a u8 (the type buffer takes
    */


}