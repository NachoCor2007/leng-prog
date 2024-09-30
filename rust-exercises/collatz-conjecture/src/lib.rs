pub fn collatz(mut n: u64) -> Option<u64> {
  if n <= 0 {
    return None
  }

  let mut steps = 0;

  while n > 1 {
    steps += 1;
    let remainder = n % 2;

    match remainder {
      0 => { n = n / 2 }
      _ => { n = 3 * n + 1 }
    }
  }

  Some(steps)
}
