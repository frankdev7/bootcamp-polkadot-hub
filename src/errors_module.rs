pub mod errors_module {
  use std::fmt;
  use std::fs::File;
  use std::io::{Read, self, ErrorKind};

  pub fn errors_1() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the file: {:?}", error),
    };

  }

  pub fn errors_2() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
          panic!("Problem opening the file: {:?}", other_error)
        }
      }
    };
  }

  pub fn errors_3() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
          panic!("Problem creating the file: {:?}", error);
        })
      } else {
        panic!("Problem opening the file: {:?}", error);
      }
    });
  }

  pub fn errors_4() {
    // let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt").expect("hello.txt shoud be included in this project");
  }

  pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
      Ok(file) => file,
      Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => {
          println!("My username: {}", username);
          Ok(username)
        },
        Err(e) => Err(e),
    }
  }

  pub fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username);
    
    println!("My username is: {}", username);
    Ok(username)
  }
  
}