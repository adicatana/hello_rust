extern crate hello_rust;

use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
use std::io;
use std::fmt::Display;

// use hello_rust::guessing_game;
// use hello_rust::fibonacci;

const FILE_IN: &str = "text.in";

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}

pub fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {

    // let mut f = File::open(file_name)?;
    // let f = File::open(file_name);

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
    // match f.read_to_string(&mut s) {
        // Ok(_) => Ok(s),
        // Err(e) => Err(e),
    // }

}

fn custom_read() {
    let f = File::open(FILE_IN);

    let _f = match f {
        Ok(file) => file,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            match File::create(FILE_IN) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create a file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(e) => {
            panic!(
                "There was a problem opening the file: {:?}",
                e
            )
        }
    };

    let _f = File::open(FILE_IN).unwrap();
    let _f = File::open(FILE_IN).expect("No such file.");
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub trait MyTest {
    fn test_function(&self) -> &str {
        "Default implementation."
    }
}

pub struct MyStruct {
    pub x: u32,
    pub y: String,
}

impl MyTest for MyStruct {
    fn test_function(&self) -> &str {
        println!("Test function call.");
        &self.y
    }
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn mock_function<T: MyTest + Display>(item: T) {
    item.test_function();
}

fn main() {

    let my_struct = MyStruct { x: 4, y: String::from("Test string") };

    let res = my_struct.test_function();

    println!("{:?}", res);

    // guessing_game::guessing_game();

    // let fib_it = fibonacci::nth_fib_log(80);
    // println!("{}", fib_it);

    // let mut x = plus_one(None);
    // println!("{:?}", x);
    //
    // if let Some(i) = x {
    //     println!("{:?}", i);
    // } else {
    //     println!("{:?}", "Tried to unwrap None.");
    // }
    //
    // x = plus_one(Some(5));
    // println!("{:?}", x);
    //
    // if let Some(i) = x {
    //     println!("{:?}", i);
    // }

}
