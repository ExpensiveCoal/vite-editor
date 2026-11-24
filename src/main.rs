use std::env;
use std::path::Path;

mod save;

fn main() -> Result<(), ()> {
    // Get the user's arguments
    let args: Vec<String> = env::args().collect();

    // Check if arguments were provided
    match args.get(1) {
        Some(_) => {}
        None => {
            println!("Please select a file to edit.");
            return Err(());
        }
    }

    // Create the path from the user's arguments
    let path: &Path = Path::new(&args[1]);

    match save::save(path) {
        Ok(_) => {
            println!("File {} successfully saved.", path.display());
        }
        Err(message) => {
            println!("{}", message);
        }
    }

    Ok(())
}