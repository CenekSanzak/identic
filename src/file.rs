use walkdir::WalkDir;
use std::path::Path;
#[derive(Debug)]
pub struct FileStruct{ // Struct that holds the file information.
    pub name: String,
    pub path: String,
    pub is_file: bool,
}
impl FileStruct{ // Constructor for the FileStruct struct.
    pub fn new(path: String, is_file: bool) -> FileStruct{
        FileStruct{
            name: get_file_name(&path),
            path,
            is_file,
        }
    }
}

pub fn get_files(path: &Path) -> Result<Vec<FileStruct>, &'static str> {
    // Gets the files in the path and returns them as a vector of FileStruct structs.
    let files = WalkDir::new(&path);
    let files = files
        .into_iter()
        .filter_map(|f| f.ok())
        .map(|f| FileStruct::new(String::from(f.path().to_string_lossy()), f.metadata().unwrap().is_file()))
        .collect::<Vec<_>>();
    Ok(files) 
}

pub fn get_file_name(path: &String) -> String {
    // split the path by the / and get the last element
    let path_split = path.split("/");
    let last_element = path_split.last().unwrap();
    String::from(last_element)
}

pub mod hashing{
    // Contains the functions that create the hash of a file.
    use std::{error::Error, fs::File, io::{BufReader, Read}};
    use crate::helpers;
    use super::FileStruct;
    use sha2::{Sha256, Digest};

    macro_rules! hash_all { // Macro that creates a hash using arbitrary number of arguments
        ($($args:expr),*) => {{
        let mut hasher = Sha256::new();
            $(
                hasher.update($args);
            )*
        format!("{:x}", hasher.finalize())
        }}
    }

    pub fn create_hash(file: &FileStruct, config: &helpers::config::Config) -> Result<String, Box<dyn Error>> {
        // Creates a hash of the file. Uses the hash_all macro to create the hash from the concatenation of hashes of the file name, and the file content.
        let hash1 = match config.name {
            true => hash_str(&file.name),
            false => String::from("")
        };
        let hash2 = match config.content {
            true => get_file_content_hash(&file),
            false => String::from("")
        };
        let concatenated_hash = hash_all!(hash1, hash2);
        Ok(concatenated_hash)
    }

    pub fn hash_str(input: &str) -> String {
        // Gets a string and returns a hash of it as a string.
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn get_file_content_hash(file: &FileStruct) -> String {
        // Gets a file and returns a hash of its content as a string.
        match file.is_file {
            true => {
                let f = File::open(&file.path).unwrap();
                    
                let mut reader = BufReader::new(f);
                let mut hasher = Sha256::new();
                let mut buffer = [0; 1024];
                loop {
                    let bytes_read = reader.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    hasher.update(&buffer[..bytes_read]);
                }
                let hash = hasher.finalize();
                let hash_str = format!("{:x}", hash);
                hash_str
            },
            false => String::from("")
        }
    }

}
