# IDENTIC
## What is Identic?

Identic was a homework assignment for the CMPE230 Systems Programming course at the Bogazici University. The assignment was to create a command line tool that could be used to find replicas of a file in specified directories. The tool was implemented in python and was tested on a Linux system. I decided to create a rust version of the tool as well. My main objective of this project is to learn rust and how to use it in a real world application. 

It includes following features of rust:
* Creating a macro 
* Creating a struct
* Implementing a constructor for the struct
* Using the `Option` type
* Using the `Result` type
* Structuring the code using modules
* Walkdir library 
* Hashmap library
* Creating sha256 hashes of file contents and names
* Iterating through a vector, filtering and mapping values
* Moving, ownership and borrowing



## Folder Structure
* Cargo.toml : TOML file for the project
* src/ : Source folder
    * main.rs : Main file for the project
    * lib.rs : Library file for the project
    * file.rs : file module that contains a struct and some helper functions for files
    * helpers/ : helpers module that contains 2 helper modules
        * mod.rs : helper module
        * config.rs : config module that contains a struct for configuring the project
        * output.rs : output module that contains functions for outputting information to the user
* test_cases/ : Test cases folder
    * file: "g"
    * folder/ 
        * file_in_folder: "g"
        * folder_in_folder/
            * file_in_folder: "g"
            * folder/
