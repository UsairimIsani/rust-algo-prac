pub fn fib(n: u32) -> u32 {
    if n > 2 {
        fib(n - 1) + fib(n - 2)
    } else {
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;
    #[test]
    fn check_fib() {
        assert_eq!(89, fib(10));
    }
}
