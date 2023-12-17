use std::{
    fs::File,
    io::{self, ErrorKind, Read, Write},
};

fn read_username_from_file() -> Result<File, io::Error> {
    // let mut username = String::new();

    let file = File::open("now.txt")?;

    Ok(file)
}
fn main() {
    let file = File::open("hello.txt");

    let mut matching_file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error create file {:?}", error),
            },
            _ => {
                panic!("Other error {:?}", error)
            }
        },
    };
    let mut content = String::new();

    // read content of file to content variable
    match matching_file.read_to_string(&mut content) {
        Ok(_) => {
            //check content of file
            if content == "" {
                content = String::from("Hello world");
                // write content to file
                match matching_file.write_all(content.as_bytes()) {
                    Ok(_) => println!("Write file success"),
                    Err(error) => panic!("Error write file {:?}", error),
                };
            }
        }
        Err(error) => panic!("Error read file {:?}", error),
    };

    let file_1 = read_username_from_file();

    println!("File 1 {:?}", file_1);

    println!("File: {:?}", matching_file);
    println!("Content: {:?}", content)
}
