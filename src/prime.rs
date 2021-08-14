pub struct PrimeGen {
    prime_idx: usize,
    primes: Vec<usize>,
}

impl PrimeGen {
    pub fn reset(&mut self) {
        self.prime_idx = 0;
    }
    pub fn factor(&mut self, mut num: usize) -> Vec<usize> {
        if num == 0 || num == 1 {
            vec![num]
        } else {
            let stored_idx = self.prime_idx;

            let mut factors = Vec::new();
            self.reset();
            while num != 1 {
                let prime = self.next().unwrap();
                while num % prime == 0 {
                    num /= prime;
                    factors.push(prime);
                }
            }
            self.prime_idx = stored_idx;

            factors
        }
    }
}

impl Default for PrimeGen {
    fn default() -> PrimeGen {
        PrimeGen {
            prime_idx: 0,
            primes: vec![2],
        }
    }
}

impl Iterator for PrimeGen {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.prime_idx + 1 >= self.primes.len() {
            let mut num = self.primes[self.primes.len() - 1];
            loop {
                num += 1;
                let sqrt = (num as f64).sqrt().ceil() as usize;
                if !self
                    .primes
                    .iter()
                    .take_while(|&&prime| prime <= sqrt)
                    .any(|&prime| num % prime == 0)
                {
                    break;
                }
            }
            self.primes.push(num);
        }
        self.prime_idx += 1;
        Some(self.primes[self.prime_idx - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::PrimeGen;

    #[test]
    fn factor_test() {
        let mut prime_gen = PrimeGen::default();

        assert_eq!(prime_gen.factor(1234), vec![2, 617]);
        assert_eq!(prime_gen.factor(100), vec![2, 2, 5, 5]);
        assert_eq!(prime_gen.factor(1), vec![1]);
        assert_eq!(prime_gen.factor(0), vec![0]);
    }

    #[test]
    fn prime_gen_test() {
        let mut prime_gen = PrimeGen::default();

        assert_eq!(prime_gen.next(), Some(2));
        assert_eq!(prime_gen.next(), Some(3));
        assert_eq!(prime_gen.next(), Some(5));
        assert_eq!(prime_gen.next(), Some(7));
        assert_eq!(prime_gen.next(), Some(11));
        assert_eq!(prime_gen.next(), Some(13));
        assert_eq!(prime_gen.next(), Some(17));
        assert_eq!(prime_gen.next(), Some(19));
        assert_eq!(prime_gen.next(), Some(23));
        assert_eq!(prime_gen.next(), Some(29));
    }
}
