///# Factorial Is A Function That calculates what it says
/// Do Understan Factorial We must first understand the Meaning iof factorial and Decide
/// So it is required by all people to discover factorial
/// * `n` is a An Argumant to be taken
/// ```
/// This is a an Example
/// let a = factorial(5); // 120
/// ```

pub fn factorial(n: u32) -> u32 {
    if n > 1 {
        n * factorial(n - 1)
    } else {
        n
    }
}
