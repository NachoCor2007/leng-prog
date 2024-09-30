type Fraction = (i32, i32);

/// Add 2 fractions
pub fn add((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let n_a = n1 * d2;
    let n_b = n2 * d1;

    let new_n = n_a + n_b;
    let new_d = d1 * d2;

    simplify(new_n, new_d)
//  (new_n, new_d)
}

/// Subtract 2 fractions
pub fn sub((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    add((n1, d1), (-n2, d2))
}

/// Multiply 2 fractions
pub fn mul((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let new_n = n1 * n2;
    let new_d = d1 * d2;

    simplify(new_n, new_d)
//  (new_n, new_d)
}

/// Divide 2 fractions
pub fn divide((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    mul((n1, d1), (d2, n2))
}

/// Calculate the Highest common factor between 2 numbers
pub fn hcf(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b
    } else if b == 0 {
        return a
    }

    let remainder = a % b;
    if remainder == 0 {
        b
    } else {
        hcf(b, remainder)
    }
}

/// Create a fraction simplifying with the arguments simplified by the `hcf`
pub fn simplify(n: i32, d: i32) -> Fraction {
    let factor = hcf(n, d);
    (n / factor, d / factor)
}
