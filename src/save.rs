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

    // Create file if it doesn't exists
    } else if !path.exists() {

        // Handle errors if the code can't create the file  
        File::create(path).map_err(|e| e.to_string())?;
        
    }

    Ok(())
}