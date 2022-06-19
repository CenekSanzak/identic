use std::{error::Error, path::Path, collections::HashMap};
use file::{hashing::create_hash, get_files};
pub mod helpers;
mod file;


// Gets the paths of the folders to compare. 
pub fn run(config: helpers::config::Config) -> Result<(), Box<dyn Error>> {
    let mut hash_map = HashMap::<String, Vec<String>>::new();
    for path in &config.paths{
        // Gets the files in the path for each path
        let files = get_files(Path::new(&path))?;
        let files = match config.content{
            true => files.into_iter().filter(|f| f.is_file).collect(),
            false => files
        }; // Filters out directories if the content flag is true. 
        for entry in files { // For each file in the path 
            // Creates a hash of the file and adds it to the hash map.
            let hash = create_hash(&entry, &config)?;
            if hash_map.contains_key(&hash) {
                hash_map.get_mut(&hash).unwrap().push(entry.path);
            } else {
                hash_map.insert(hash.clone(), vec![entry.path]);
            }
        }
    }
    helpers::output::print_results(hash_map)?; // Prints the results.
    Ok(())
}

