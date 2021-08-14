pub fn reverse_digits(mut n: usize, base: usize) -> usize {
    let mut reversed = 0;

    while n > 0 {
        reversed = (reversed * base) + (n % base);
        n /= base;
    }

    reversed
}

pub fn is_palindrome(n: usize, base: usize) -> bool {
    reverse_digits(n, base) == n
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub fn n_digits(n: usize, base: usize) -> usize {
    (n as f64).log(base as f64) as usize + 1
}

pub fn digits_iter(mut n: usize, base: usize) -> impl Iterator<Item = usize> {
    let n_digits = n_digits(n, base);
    (0..).take(n_digits).map(move |_| {
        let d = n % base;
        n /= base;
        d
    })
}

pub fn sum_to_n(n: usize) -> usize {
    n * (n + 1) / 2
}
pub fn sum_squares_to_n(n: usize) -> usize {
    n * (n + 1) * (2 * n + 1) / 6
}
pub fn sum_cubes_to_n(n: usize) -> usize {
    let sum_n = sum_to_n(n);
    sum_n * sum_n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_base_10() {
        let b = 10;

        assert_eq!(reverse_digits(10, b), 1);
        assert_eq!(reverse_digits(101, b), 101);
        assert_eq!(reverse_digits(1234, b), 4321);
        assert_eq!(reverse_digits(101827, b), 728101);
    }

    #[test]
    fn reverse_base_16() {
        let b = 16;

        assert_eq!(reverse_digits(0xabcdef, b), 0xfedcba);
        assert_eq!(reverse_digits(0x4321, b), 0x1234);
        assert_eq!(reverse_digits(0xab0e21, b), 0x12e0ba);
    }
}
