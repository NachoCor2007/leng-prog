pub fn collatz(mut n: u64) -> Option<u64> {
  let remainder = n % 2;
  match remainder {
    0 => {
      let mut count = 0;
      while n != 1 {
        n = n / 2;
        count += 1;
      }
      Some(count)
    },
    1 => {
      let mut count = 0;
      while n != 1 {
        n = 3 * n + 1;
        count += 1;
      }
      Some(count)
    },
    _ => None
  }
}
