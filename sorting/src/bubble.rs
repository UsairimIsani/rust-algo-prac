// pub fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
//     let mut sorted_vector: Vec<T> = collection.into();
//     for _ in 0..sorted_vector.len() {
//         let mut swaps = 0;
//         for i in 1..sorted_vector.len() {
//             if sorted_vector[i - 1] > sorted_vector[i] {
//                 sorted_vector.swap(i - 1, i);
//                 swaps += 1;
//             }
//         }
//         if swaps == 0 {
//             break;
//         }
//     }
//     sorted_vector
// }
pub fn bubble_sort<T>(col: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut sorted_vector: Vec<T> = col.into();
    for _ in 0..sorted_vector.len() {
        let mut swaps = 0;
        for i in 1..sorted_vector.len() {
            if sorted_vector[i - 1] > sorted_vector[i] {
                sorted_vector.swap(i - 1, i);
                swaps = swaps + 1;
            }
        }
        if swaps == 0 {
            break;
        }
    }
    sorted_vector
}
