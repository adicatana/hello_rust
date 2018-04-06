extern crate hello_rust;

// use hello_rust::guessing_game;
// use hello_rust::fibonacci;

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}

fn main() {

    

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
