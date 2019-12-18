///# My Failed Attempt to Do Merge Sort Without Looking
// pub fn merge_sort<T: PartialOrd + Clone + std::fmt::Debug>(arr: &[T]) -> Vec<T> {
//     let arr: Vec<T> = arr.into();
//     let arr_len = arr.len();
//     println!("{} Array length", arr_len);
//     if arr_len > 1 {
//         let (left_arr, right_arr) = arr.split_at(arr_len / 2);
//         println!("{:?} {:?}", &left_arr, &right_arr);
//         merger(&merge_sort(left_arr), &merge_sort(right_arr))
//     } else {
//         arr
//     }
// }
// pub fn merger<T: PartialOrd + Clone + std::fmt::Debug>(left_arr: &[T], right_arr: &[T]) -> Vec<T> {
//     let mut left_arr: Vec<T> = left_arr.into();
//     let mut right_arr: Vec<T> = right_arr.into();
//     let mut result: Vec<T> = vec![];
//     let left_len = left_arr.len();
//     let right_len = right_arr.len();
//     println!("{} Left Len {} Right Len", left_len, right_len);
//     let ran = if left_len <= right_len {
//         left_len
//     } else {
//         right_len
//     };
//     let mut i = 0;
//     let mut j = 0;
//     while i < left_arr.len() && j < right_arr.len() {
//         println!("{} Range", &i);
//         if left_arr[i] < right_arr[i] {
//             &result.push(left_arr[i].clone());
//             &left_arr.remove(i);
//         } else if left_arr[i] > right_arr[i] {
//             &result.push(right_arr[i].clone());
//             &right_arr.remove(i);
//         }
//     }
//     if left_arr.len() > 0 {
//         &result.append(&mut left_arr);
//     }
//     if right_arr.len() > 0 {
//         &result.append(&mut right_arr);
//     }
//     println!("{:?}", &result);
//     result
// }
// #[cfg(test)]
// #[test]
// fn test_merge_sub() {
//     assert_eq!(vec![1, 2], merge_sort(&[2, 1]));
// }
// #[test]
// fn test_merge_sort() {
//     assert_eq!(vec![1, 2, 3, 4, 5, 78], merge_sort(&[2, 4, 5, 78, 1, 3]))
// }

// pub fn merge_sort<T: PartialOrd + Clone + std::fmt::Debug>(collection: &[T]) -> Vec<T> {
//     if collection.len() > 1 {
//         let (l, r) = collection.split_at(collection.len() / 2);
//         let sorted_l = merge_sort(l);
//         let sorted_r = merge_sort(r);
//         let mut result: Vec<T> = collection.into();
//         let (mut i, mut j) = (0, 0);
//         let mut k = 0;
//         while i < sorted_l.len() && j < sorted_r.len() {
//             if sorted_l[i] <= sorted_r[j] {
//                 result[k] = sorted_l[i].clone();
//                 i += 1;
//             } else {
//                 result[k] = sorted_r[j].clone();
//                 j += 1;
//             }
//             k += 1;
//         }
//         while i < sorted_l.len() {
//             result[k] = sorted_l[i].clone();
//             k += 1;
//             i += 1;
//         }
//         while j < sorted_r.len() {
//             result[k] = sorted_r[j].clone();
//             k += 1;
//             j += 1;
//         }
//         result
//     } else {
//         collection.to_vec()
//     }
// }

pub fn my_merge_sort<T: PartialOrd + Clone + std::fmt::Debug>(collection: &[T]) -> Vec<T> {
    if collection.len() > 1 {
        let mut result: Vec<T> = collection.into();
        let (l, r) = collection.split_at(collection.len() / 2);
        let sor_l = my_merge_sort(l);
        let sor_r = my_merge_sort(r);
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < sor_l.len() && j < sor_r.len() {
            if sor_l[i] <= sor_r[j] {
                result[k] = sor_l[i].clone();
                i += 1;
            } else {
                result[k] = sor_r[j].clone();
                j += 1;
            }
            k += 1;
        }
        while i < sor_l.len() {
            result[k] = sor_l[i].clone();
            i += 1;
            k += 1;
        }
        while j < sor_r.len() {
            result[k] = sor_r[j].clone();
            j += 1;
            k += 1;
        }

        result
    } else {
        collection.to_vec()
    }
}
