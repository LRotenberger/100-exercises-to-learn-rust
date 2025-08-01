// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // solutions showed you can override 'n' as a usize variable to remove the 4 casts
    // i.e. 'let n = n as usize;'
    // as well as initializing the vector with 0 and 1 instead of giving it initial capacity -- Note: next lesson mentioned using 'with_capacity' to avoid memory reallocation
    // i.e. 'let mut fib = vec![0, 1];'
    let mut fib: Vec<u32> = Vec::with_capacity((n) as usize);
    fib.push(0);
    fib.push(1);
    for i in 2..n+1 {
        fib.push(fib[(i-2) as usize] + fib[(i-1) as usize]);
    }
    fib[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
