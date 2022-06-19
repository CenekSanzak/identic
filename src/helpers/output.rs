use std::error::Error;
use std::collections::HashMap;

// Prints a list of files that have the same hash (identical files).
pub fn print_identical_files(vec_of_paths: Vec<String>) -> () {
    println!("-------------");
    for path in vec_of_paths.iter(){
        println!("{}", &path)
    }
}

// Prints the list of files that have the same hash (identical files).
pub fn print_results(hash_map: HashMap::<String, Vec<String>>) ->  Result<(), Box<dyn Error>>{
    hash_map.into_values()
    .into_iter()
    .filter(|v| v.len()>1) // Filter out hashes that have only one file.
    .for_each(|v| print_identical_files(v));
    Ok(())
}