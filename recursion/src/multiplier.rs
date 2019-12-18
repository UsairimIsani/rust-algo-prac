///! Hello World
///# Chicken
/// * Hello World
/// - [] Hel
/// - [] sd
/// ```
/// let a = multiplier(vec![1,2,4,5,6]) -> Something?
/// ```
pub fn multiplier(vc: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..vc.len() {
        vc[i] = vc[i] * i as i32 + 1;
    }
    vc
}
#[cfg(test)]
#[test]
fn mult() {
    println!("{:?}", multiplier(&mut vec![1, 2, 6, 7, 8, 3, 4, 563, 789]));
    assert_eq!(&mut vec![2, 4, 6], multiplier(&mut vec![1, 2, 3]));
    // assert_eq!(&vec![2, 4, 6, 8, 10], multiplier(&mut vec![1, 2, 3, 4, 5]));
}
