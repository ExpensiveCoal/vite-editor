use std::path::Path;
use std::fs::File;

pub fn save(path: &Path) -> Result<(), std::io::Error> {
    // Check if file exists and isn't a directory
    if path.is_file() {
        return Ok(());
 
    } else if path.is_dir(){

        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,

            format!("'{}' is a directory.", path.display())

        ));
        
    // Create file if it doesn't exists
    } else if !path.exists() {

        // Handle errors if the code can't create the file  
        File::create(path)?;
        
    }

    Ok(())
}