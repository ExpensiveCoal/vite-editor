use std::path::Path;
use std::fs::File;

pub fn save(path: &Path) -> Result<(), String> {
    // Check if file exists and isn't a directory
    if path.is_file() {
        return Ok(());
 
    } else if path.is_dir(){

        return Err(String::from(format!(

            "'{}' is a directory.",

            path.display()

        )));

    // Create the file if it doesn't exists
    } else if !path.exists() {

        // Handle errors if the code can't create the file  
        match File::create(path) {

            Ok(_) => {
                return Ok(());

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
        return Err(String::from("Couldn't find, open or create file, try running code as root."));
    }
}