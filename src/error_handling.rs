use std::fs::File;
use std::io::ErrorKind;

pub fn error_handling() {
    let greeting_file_result = File::open("hello.txt");
    // let greeting_file1= match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };
    // Matching on different errors
    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(error) => panic!("Problem creating the file: {error:?}"),
        },
        other_error => {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
