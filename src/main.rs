use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, event, execute, terminal, queue};
use crossterm::terminal::ClearType;
use std::io::{stdout, Write, self};
use std::time::Duration;

const VERSION: &str = "0.0.1";

struct CleanUp; // struct for cleaning up incase program panics and cannot exit raw mode

// method to drop raw mode in event of panic
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Couldn't disable raw mode.");
        Output::clear_screen().expect("Error.");
    }
}

//struct to handle output to screen
struct Output {
    win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0,0))
    }

    fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.editor_contents,
            cursor::Hide,
            cursor::MoveTo(0,0)
        )?;

        self.draw_rows();

        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;

        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;

        self.editor_contents.flush()
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;

        for i in 0..screen_rows {

            if i == screen_rows / 3 {
                let mut welcome = format!("Super Editor --- Version {}", VERSION);
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns)
                }

                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome);
            } else {
                self.editor_contents.push('~');
            }

            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            ).unwrap();

            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n")
            }
        }
    }

    fn move_cursor(&mut self, direction:KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {

    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buffer) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }

}


// struct for reading key presses
struct Reader;

impl Reader {
    // read key function similar to our loop we had before
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }

}

// Editor is responsible for running our program much as a main function would
struct Editor {
    reader:Reader,
    output:Output,
}

impl Editor {

    // new method to create a new instance of Editor
    fn new() -> Self {
        Self { 
            reader: Reader,
            output: Output::new(),
        }

    }

    // return whether or not program will continue (was ctrl + q entered)
    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {

            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),

            KeyEvent {
                code: direction @ 
                (
                KeyCode::Up 
                | KeyCode::Down 
                | KeyCode::Left 
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End
                ),
                modifiers: KeyModifiers::NONE,
            } => self.output.move_cursor(direction),

            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => (0..self.output.win_size.1).for_each(|_| {
                self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),

            _ => {}
        } 
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}

struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    screen_columns: usize,
    screen_rows: usize,
}

impl CursorController {
    fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
        }
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y != self.screen_rows - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x != self.screen_columns - 1 {
                    self.cursor_x += 1;
                }
            }

            KeyCode::End => self.cursor_x = self.screen_columns - 1,
            KeyCode::Home => self.cursor_x = 0,

            _ => unimplemented!(),
        }
    }
}


fn main() -> crossterm::Result<()> { // this means main will return a Result {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?; // changed to shorthand for expect (? can only be used in a method that returns a Result or Option)

    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())

}