use std::env;
use std::path::Path;

mod save;
mod editor;

fn main() -> Result<(), std::io::Error> {
    // Get the user's arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument was provided
    let filename: &String = match args.get(1) {
        Some(filename) => filename,
        None => {
            println!("Please select a file to edit.");
            return Ok(());
        }
    };

    // Create the path from the user's arguments
    let path: &Path = Path::new(filename);
    
    match editor::editor(path) {
        Ok(_) => {}
        
        Err(message) => {
            println!("{}", message);
        }
    }

    Ok(())
}