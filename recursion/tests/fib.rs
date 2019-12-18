extern crate recursion;
use recursion::fib::fib;
#[cfg(test)]
#[test]
fn checking_fib() {
    assert_eq!(89, fib(10));
}
