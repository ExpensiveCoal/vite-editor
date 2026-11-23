use std::env;
use std::path::Path;
use std::fs::File;

fn main() -> Result<(), String> {

    // Get the user's argurments 
    let args: Vec<String> = env::args().collect();

    // Check if the arguments were provided
    match args.get(1) {
        Some(_) => {}
        None => return Err(String::from("Please select a file to edit."))
    }

    // Create the path from the user's arguments
    let path: &Path = Path::new(&args[1]);

    // Check if file exists and isn't a directory
    if path.is_file() {
        println!("File '{}' exists.", path.display());
 
    } else if path.is_dir(){

        return Err(String::from(format!(

            "'{}' is a directory.",

            path.display()

        )));

    // Create the file if it doesn't exists
    } else if !path.exists() {

        println!("File '{}' does not exists, creating it...", path.display());

        // Handle errors if the code can't create the file  
        match File::create(path) {

            Ok(_) => {
                println!("Successfully created successfully file '{}'!", path.display())
            }

            Err(error) => {

                return Err(String::from(format!(

                    "Couldn't create file '{}', error message: {}",

                    path.display(),

                    error.to_string()

                )));
            }
        }
    } else {
        return Err(String::from("Error, try running the code as root."));
    }

    Ok(())
}