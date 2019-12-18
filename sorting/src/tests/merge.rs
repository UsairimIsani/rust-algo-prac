#[cfg(test)]
#[test]
pub fn test_merge_sort() {
    extern crate sorting;
    use sorting::merge::my_merge_sort;
    assert_eq!(vec![1, 2, 3, 4, 5, 78], my_merge_sort(&[2, 4, 5, 78, 1, 3]));
}
