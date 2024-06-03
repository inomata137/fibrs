use rug::Integer;
use std::collections::VecDeque;

type FibPair = (Integer, Integer);

fn square(pair: &FibPair) -> FibPair {
    let a = Integer::from(&pair.0 * &pair.0);
    let b = Integer::from(&pair.0 * &pair.1);
    let c = Integer::from(&pair.1 * &pair.1);
    (2 * b - &a, a + c)
}

fn square_and_increment(pair: &FibPair) -> FibPair {
    let a = Integer::from(&pair.0 * &pair.0);
    let b = Integer::from(&pair.0 * &pair.1);
    let c = Integer::from(&pair.1 * &pair.1);
    (a + &c, 2 * b + c)
}

fn to_binary(n: u64) -> VecDeque<bool> {
    let mut n = n;
    let mut expr = VecDeque::<bool>::new();
    while n > 0 {
        expr.push_back(n % 2 == 1);
        n >>= 1;
    }
    expr
}

pub fn fibonacci(n: u64) -> Integer {
    let mut bin_expr = to_binary(n);
    let mut state: FibPair = (Integer::from(0u8), Integer::from(1u8));
    while let Some(d) = bin_expr.pop_back() {
        if d {
            state = square_and_increment(&state);
        } else {
            state = square(&state);
        }
    }
    state.0
}
