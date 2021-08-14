pub struct Fibonacci {
    a: usize,
    b: usize,
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        std::mem::swap(&mut self.a, &mut self.b);
        self.b += self.a;
        Some(self.a)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { a: 0, b: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::Fibonacci;

    #[test]
    fn first_5() {
        let mut f = Fibonacci::default();
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        assert_eq!(f.next(), Some(5));
        assert_eq!(f.next(), Some(8));
    }
}
