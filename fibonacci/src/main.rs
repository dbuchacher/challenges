/*
    Generate the nth Fibonacci number.
*/

// find fibonacci number
fn fib(n: u32) -> u32 {
    // special cases for 0 and 1
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    // return fibonacci formula
    fib(n-1) + fib(n-2)
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(20), 6765);
}