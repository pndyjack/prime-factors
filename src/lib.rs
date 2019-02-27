pub fn factors(n: u64) -> Vec<u64> {
  if n == 0 || n == 1 {
    return vec![];
  }

  let mut number = n;
  let mut factors = vec![];
  for factor in (2..).take_while(move |&i| i <= (number / 2) + 1) {
    while number % factor == 0 {
      number /= factor;
      factors.push(factor);
    }
    if number == 1 {
      break;
    }
  }

  factors
}
