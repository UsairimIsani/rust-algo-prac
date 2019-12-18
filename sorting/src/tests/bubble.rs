extern crate sorting;
use sorting::bubble::bubble_sort;
#[cfg(test)]
#[test]
fn sort_via_bubble() {
    assert_eq!(vec![1, 2, 3, 4, 5], bubble_sort(vec![4, 2, 1, 5, 3]));
}
