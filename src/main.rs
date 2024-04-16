use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc)=> fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),

    //         },
    //         other_error =>{
    //             panic!("Problem opening the file: {:?}",other_error);
    //         }
    //     },
    // };

    // let my_greetings_File = File::open("birthday.txt").unwrap();

    // let my_greetings_expect_file = File::open("birthday.txt").expect("birthday.txt is not in your project");

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("birthday.txt")
    }
}

