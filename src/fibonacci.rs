// Square matrix 2 by 2
// a b
// c d
#[derive(Debug)]
struct SquareMatrix {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
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

    fn nth_power(&self, n: u64) -> SquareMatrix {
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

pub fn nth_fib_log(n: u64) -> u64 {
    let mut fib_m = SquareMatrix::fib_matrix();
    let init_fib_m = SquareMatrix::initial_fib();
    fib_m = fib_m.nth_power(n);
    fib_m = fib_m.mult(&init_fib_m);
    fib_m.a
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

pub fn nth_fib_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let aux = a;
        a = b;
        b += aux;
    }
    a
}

pub fn nth_fib_recursive(n: u64) -> u64 {
    if n == 0 || n == 1 {
        n
    } else {
        nth_fib_recursive(n - 1) + nth_fib_recursive(n - 2)
    }
}
