use recursion::factorial::factorial;
// use recursion::*;
#[cfg(test)]
#[test]
fn checking_factorial() {
    assert_eq!(120, factorial(5));
}
