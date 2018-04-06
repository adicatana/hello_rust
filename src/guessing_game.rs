extern crate rand;
extern crate std;

use self::rand::Rng;

const MAX_LOOP: u64 = 50;

pub fn guessing_game() {
    println!("Guess the number.");

    let random_number = rand::thread_rng().gen_range(1, 101);

    for _ in 1..MAX_LOOP {

        let mut guess = make_guess();

        println!("You guessed {}", guess);

        let guess : u64 = match guess.trim().parse() {
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
