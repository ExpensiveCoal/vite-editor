use std::path::Path;
use crossterm::{
    cursor::MoveTo, event::{self, Event, KeyCode, KeyModifiers}, execute, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
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

    loop {
        // Run when user type a key
        if let Event::Key(key) = event::read()? {
            // Check if Ctrl key was press
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Check if key 'x' was press with Ctrl
                if key.code == KeyCode::Char('x') {
                    // Exit loop without saving the file
                    break;
                }

                if key.code == KeyCode::Char('o') {
                    // Save file and exit loop
                    crate::save::save(path)?;
                    break;
                }

                // Ignore other Ctrl combination
                continue;
            }
        }
    }

    // Restore normal terminal mode
    disable_raw_mode()?;

    Ok(())
}