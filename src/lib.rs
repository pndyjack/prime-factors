pub fn factors(n: u64) -> Vec<u64> {
  if n == 0 || n == 1 {
    return Vec::new();
  }
  let mut factors: Vec<u64> = Vec::new();
  let mut number = n;
  for factor in 2..=n {
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
