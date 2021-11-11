use std::fs::File;
use std::io;
// use std::io::{ErrorKind, Read};
use std::io::{Read};

// pub fn test() {
//     let file_res = File::open("test");
//     let _file = match file_res {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error)
//     };
// }

// pub fn test() {
//     let file_res = File::open("test");
//     let _file = match file_res {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e)
//             }
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         }
//     };
// }

// pub fn test() {
//     let _file = File::open("test").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error)
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// pub fn test() {
//     let _file = File::open("test").unwrap();
//     // let _file = File::open("test").expect("Failed to open the hello.txt");
// }

// pub fn test() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

pub fn test() -> Result<String, io::Error> {
    let mut s = String::new();
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}