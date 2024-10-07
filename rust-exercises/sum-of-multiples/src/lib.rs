pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut i = 1;
    let mut result = 0;
    while i < limit {
        let is_multiple = check_multiple(i, &factors);

        if is_multiple {
            result += i
        }

        i += 1
    }

    result
}

fn check_multiple(number: u32, factors: &[u32]) -> bool {
    for factor in factors {
        if *factor == 0 {
            continue
        }

        let rem = number % factor;

        if rem == 0 {
            return true
        }
    }

    false
}
