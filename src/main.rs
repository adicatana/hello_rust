extern crate rand;

use rand::Rng;

fn main() {
    println!("Guess the number.");

    let random_number = rand::thread_rng().gen_range(1, 101);

    loop {

	    println!("Please enter your guess:");

	    let mut guess = String::new();

	    std::io::stdin().read_line(&mut guess).
	    	expect("Failed to read line.");

	    println!("You guessed {}", guess);

	    let guess : u32 = guess.trim().parse().
	    	expect("Faled to convert to number");

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
