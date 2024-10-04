use std::ops;

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction(pub i32, pub i32);

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        let n_a = self.0 * other.1;
        let n_b = other.0 * self.1;

        let new_n = n_a + n_b;
        let new_d = self.1 * other.1;

        simplify(new_n, new_d)
    }

    pub fn sub(& self, other: Fraction) -> Fraction {
        self.add(Fraction(-other.0, other.1))
    }

    pub fn mul(& self, other: Fraction) -> Fraction {
        let new_n = self.0 * other.0;
        let new_d = self.1 * other.1;

        simplify(new_n, new_d)
    }

    pub fn divide(& self, other: Fraction) -> Fraction {
        self.mul(Fraction(other.1, other.0))
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::add(&self, rhs)
    }
}

/// Calculate the Highest common factor between 2 numbers
fn hcf(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { hcf(b, a % b) }
}

fn simplify(n: i32, d: i32) -> Fraction {
    let h = hcf(n, d);
    Fraction(n/h, d/h)
}
