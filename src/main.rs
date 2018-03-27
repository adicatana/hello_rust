extern crate rand;

use rand::Rng;

const MAX_LOOP: u32 = 100_000;  

fn main() {
    // guessing_game();
    for _ in 0..100 {
        let fib = nth_fibonacci_recursive(6);
        let fib_it = nth_fibonacci_iterative(6);
        if fib != fib_it {
            println!("Different values");
        }
    }
    // println!("{}", fib);
    // println!("{}", fib_it);
}

fn nth_fibonacci_log(n: u32) -> u32 {
    // TODO
    n
}

fn nth_fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let aux = a;
        a = b;
        b += aux;
    }
    a
}

fn nth_fibonacci_recursive(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else { 
        nth_fibonacci_recursive(n - 1) + nth_fibonacci_recursive(n - 2)
    }
}

fn guessing_game() {
    println!("Guess the number.");

    let random_number = rand::thread_rng().gen_range(1, 101);

    for _ in 1..MAX_LOOP {

        let mut guess = make_guess();

        println!("You guessed {}", guess);

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // expect("Faled to convert to number");
        };

        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You won!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}

fn make_guess() -> String {
    println!("Please enter your guess:");

    let mut guess = String::new();

	std::io::stdin().read_line(&mut guess).
	   	expect("Failed to read line.");

	guess 
}