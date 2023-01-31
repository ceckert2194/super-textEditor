use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{cursor, event, execute, terminal, queue};
use crossterm::terminal::ClearType;
use std::io::{stdout, Write, self};
use std::time::Duration;

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
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0,0))
    }

    fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.editor_contents, terminal::Clear(ClearType::All), cursor::MoveTo(0,0))?;        
        self.draw_rows();
        queue!(self.editor_contents, cursor::MoveTo(0,0))?;
        self.editor_contents.flush()
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        for i in 0..screen_rows {
            self.editor_contents.push('~');
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n")
            }
            stdout().flush();
        }
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
    fn process_keypress(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            _ => {}
        } 
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}

fn main() -> crossterm::Result<()> { // this means main will return a Result {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?; // changed to shorthand for expect (? can only be used in a method that returns a Result or Option)

    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())

}