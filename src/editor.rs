use std::{io::stdout, path::Path};
use crossterm::{
    cursor::{ MoveLeft, MoveTo, MoveToColumn},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

pub fn editor(path: &Path) -> Result<(), std::io::Error> {
    // Put the CLI in raw mode
    enable_raw_mode()?;

    // Clear the CLI
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;

    // Print the head of the editor
    print_head(path)?;

    // Sart the editor 
    editor_loop(path, String::new())?;

    // Restore normal terminal mode
    disable_raw_mode()?;

    Ok(())
}


fn print_head(path: &Path ) -> Result<(), std::io::Error> {
    // Tries to get the full file path
    let display_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let path_string = display_path.display().to_string();

    print!("{}", path_string);

    execute!(
        stdout(),
        MoveTo(0, 1)
    )?;

    print!("{}", "#".repeat(path_string.chars().count()));

    execute!(
        stdout(),
        MoveTo(0, 2)
    )?;

    Ok(())
}


fn editor_loop(path: &Path, mut text: String) -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();

    loop {
        // Execute if a key is pressed
        if let Event::Key(key) = event::read()? {
            // Execute if control key was pressed
            if key.modifiers.contains(KeyModifiers::CONTROL) {

                match key.code {
                    // Leave editor without saving
                    KeyCode::Char('x') => {
                        break;
                    }

                    // Leave editor and save
                    KeyCode::Char('o') => {
                        crate::save::save(path, text)?;

                        // Clear the CLI
                        execute!(
                            std::io::stdout(),
                            Clear(ClearType::All),
                            MoveTo(0, 0)
                        )?;

                        print!("successfully saved: {}", path.display());
                        break;
                    }

                    _ => continue,
                }
            }

            match key.code {
                // Check if a char was pressed
                KeyCode::Char(c) => {
                    // Put it in the string
                    text.push(c);
                    // Print the char in the CLI
                    print!("{}", c);
                    // Update the CLI
                    execute!(
                        stdout
                    )?;
                }

                KeyCode::Enter => {
                    // Create a new line
                    text.push('\n');

                    // Move cursor to a new line
                    execute!(
                        stdout,
                        MoveToColumn(0),
                        crossterm::cursor::MoveDown(1)
                    )?;
                }

                KeyCode::Backspace => {
                    // Delete last char
                    if text.pop().is_some() {
                        execute!(
                            stdout,
                            MoveLeft(1),
                            Clear(ClearType::UntilNewLine)
                        )?;
                    }
                }

                _ => {}
            }
        }
    }
    Ok(())
}