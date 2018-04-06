extern crate hello_rust;

use hello_rust::fibonacci;
use hello_rust::guessing_game;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}

fn main() {

    // 0 1 1 2 3 5
    // let s = String::from("test string only");

    // for i in 0..10 {
        // print!("{:?} ", nth_fib_log(i));
    // }

    // first_word(&s);

    // println!("{}", first_word(&s));

    guessing_game::guessing_game();
    // for i in 0..MAX_LOOP {
    // let fib = nth_fib_recursive(50);
    let fib_it = fibonacci::nth_fib_log(80);
    //     if fib != fib_it {
    //         println!("Different values");
    //     }
    // }


    // println!("{}", fib);
    println!("{}", fib_it);

    let mut x = plus_one(None);
    println!("{:?}", x);

    if let Some(i) = x {
        println!("{:?}", i);
    } else {
        println!("{:?}", "Tried to unwrap None.");
    }

    x = plus_one(Some(5));
    println!("{:?}", x);

    if let Some(i) = x {
        println!("{:?}", i);
    }

}
