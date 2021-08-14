use euler::fibonacci::Fibonacci;
use euler::num::{digits_iter, is_palindrome, lcm, sum_squares_to_n, sum_to_n};
use euler::prime::PrimeGen;

#[test]
fn p001() {
    // sum of natural numbers below 1000 which are multiples of 3 or 5
    let sum: usize = (0..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    assert_eq!(sum, 233_168);
}

#[test]
fn p002() {
    let sum: usize = Fibonacci::default()
        .skip(1)
        .take_while(|&n| n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum();
    assert_eq!(sum, 4_613_732);
}

#[test]
fn p003() {
    let mut gen = PrimeGen::default();
    assert_eq!(gen.factor(600851475143).into_iter().max().unwrap(), 6857);
}

#[test]
fn p004() {
    let mut max_product = 0;

    for a in 100..=999 {
        for b in 100..=999 {
            let product = a * b;

            if is_palindrome(product, 10) {
                max_product = max_product.max(product);
            }
        }
    }

    assert_eq!(max_product, 906609);
}

#[test]
fn p005() {
    assert_eq!((1..=20).reduce(lcm).unwrap(), 232_792_560);
}

#[test]
fn p006() {
    let sum_of_n = sum_to_n(100);
    let square_of_sum = sum_of_n * sum_of_n;
    let sum_of_squares = sum_squares_to_n(100);

    assert_eq!(square_of_sum - sum_of_squares, 25_164_150);
}

#[test]
fn p007() {
    let mut gen = PrimeGen::default();

    assert_eq!(gen.nth(10_000).unwrap(), 104_743)
}

#[test]
fn p008() {
    let number = concat!(
        "73167176531330624919225119674426574742355349194934",
        "96983520312774506326239578318016984801869478851843",
        "85861560789112949495459501737958331952853208805511",
        "12540698747158523863050715693290963295227443043557",
        "66896648950445244523161731856403098711121722383113",
        "62229893423380308135336276614282806444486645238749",
        "30358907296290491560440772390713810515859307960866",
        "70172427121883998797908792274921901699720888093776",
        "65727333001053367881220235421809751254540594752243",
        "52584907711670556013604839586446706324415722155397",
        "53697817977846174064955149290862569321978468622482",
        "83972241375657056057490261407972968652414535100474",
        "82166370484403199890008895243450658541227588666881",
        "16427171479924442928230863465674813919123162824586",
        "17866458359124566529476545682848912883142607690042",
        "24219022671055626321111109370544217506941658960408",
        "07198403850962455444362981230987879927244284909188",
        "84580156166097919133875499200524063689912560717606",
        "05886116467109405077541002256983155200055935729725",
        "71636269561882670428252483600823257530420752963450",
    );

    let mut greatest = 0;
    for i in 0..number.len() - 12 {
        let n = number[i..i + 13].parse().unwrap();
        let product = digits_iter(n, 10).product();
        greatest = greatest.max(product);
    }

    assert_eq!(greatest, 23_514_624_000);
}

#[test]
fn p009() {
    for a in 1..1000 {
        for b in a + 1..1000 {
            let c = 1000 - (a + b);

            if a * a + b * b == c * c {
                assert_eq!(a * b * c, 31_875_000);
            }
        }
    }
}

#[test]
fn p010() {
    let gen = PrimeGen::default();

    assert_eq!(
        gen.take_while(|&p| p < 2_000_000).sum::<usize>(),
        142_913_828_922
    );
}
