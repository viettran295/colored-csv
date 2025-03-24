use std::{error::Error, io::{self, Write}};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode, terminal_size};

pub fn show(file_content: String) -> Result<(), Box<dyn Error>>{
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin().keys();
    
    // Clear screen and move cursor to top
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::BlinkingBlock)?;

    // Get terminal size
    let (mut width, _) = terminal_size()?.into();
    
    // Get content width
    let content_width = file_content.lines()
                                            .next()
                                            .map_or(0, |line| line.len());
    
    // Adjust terminal size if content is wider
    while content_width > width as usize {
        let new_width = content_width + 10; // Add some padding
        width = new_width as u16;
    }
    write!(stdout, "\x1b[8;{}t", width)?;
    stdout.flush()?;
    
    
    // Center the content if possible
    if content_width < width as usize {
        for line in file_content.lines() {
            write!(stdout, "{}\r\n", line)?;
        }
    }
    
    stdout.flush()?;

    loop {
        match stdin.next() {
            Some(Ok(Key::Ctrl('c'))) => {
                write!(stdout, "\r\n{}\r\nExiting...\r\n", cursor::Show)?;
                stdout.flush()?;
                break;
            }
            Some(Err(e)) => eprintln!("Error: {}", e),
            None => (),
            _ => {}
        }
    }
    Ok(())
}
