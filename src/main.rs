extern crate rand;

use rand::Rng;

const MAX_LOOP: u32 = 100_000;  

// Square matrix 2 by 2
// a b
// c d
#[derive(Debug)]
struct SquareMatrix {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl SquareMatrix {
    // ------------- Associated functions -------------
    fn identity_matrix() -> SquareMatrix {
        SquareMatrix {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }

    fn fib_matrix() -> SquareMatrix {
        SquareMatrix {
            a: 0,
            b: 1,
            c: 1,
            d: 1,
        }
    }

    fn initial_fib() -> SquareMatrix {
        SquareMatrix {
            a: 0,
            b: 0,
            c: 1,
            d: 1,
        }
    }

    // ------------- Methods for square matrices -------------

    fn mult(&self, other: &SquareMatrix) -> SquareMatrix {
        SquareMatrix {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }

    fn nth_power(&self, n: u32) -> SquareMatrix {
        if n == 0 {
            SquareMatrix::identity_matrix()
        } else {
            let m = self.nth_power(n / 2);
            if n % 2 == 0 {
                m.mult(&m)
            } else {
                self.mult(&(m.mult(&m)))
            }
        }
    }
}

fn nth_fib_log(n: u32) -> u32 {
    let mut fib_m = SquareMatrix::fib_matrix();
    let init_fib_m = SquareMatrix::initial_fib();
    fib_m = fib_m.nth_power(n);
    fib_m = fib_m.mult(&init_fib_m);
    fib_m.a
}

fn main() {

    // 0 1 1 2 3 5
    // let s = String::from("test string only");

    for i in 0..10 {
        print!("{:?} ", nth_fib_log(i));
    }

    // first_word(&s);

    // println!("{}", first_word(&s));

    // guessing_game();
    // for i in 0..40 {
    //     let fib = nth_fib_log(i);
    //     let fib_it = nth_fib_iterative(i);
    //     if fib != fib_it {
    //         println!("Different values");
    //     }
    // }
    // println!("{}", fib);
    // println!("{}", fib_it);
}

#[allow(dead_code)]
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

#[allow(dead_code)]
fn nth_fib_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let aux = a;
        a = b;
        b += aux;
    }
    a
}

#[allow(dead_code)]
fn nth_fib_recursive(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else { 
        nth_fib_recursive(n - 1) + nth_fib_recursive(n - 2)
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn make_guess() -> String {
    println!("Please enter your guess:");

    let mut guess = String::new();

	std::io::stdin().read_line(&mut guess).
	   	expect("Failed to read line.");

	guess 
}