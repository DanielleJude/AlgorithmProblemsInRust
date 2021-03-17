/// Given an array of integers that are out of order,
/// determine the bounds of the smallest window
/// that must be sorted in order for the entire array
/// to be sorted
mod smallest_window {
    use std::cmp::{max, min};

    /// O(n) time, O(1) space
    pub fn get_smallest_window_traverse_both_ends(arr : &Vec<i32>) -> (usize, usize) {
        if arr.len() <= 1 {
            // add errors later...
            return (0, 0);
        }

        // traversing from left to right find the last element that is less than the running max
        // which would be our right index
        let mut max_seen : i32 = arr[0];
        let mut right : usize = 0;
        for i in 0..arr.len() {
            max_seen = max(max_seen, arr[i]);
            if arr[i] < max_seen {
                right = i;
            }
        }

        // traversing from right to left, find the left index,
        // which is the last (but first in normal traversal) where the value is greater
        // than the running min
        let mut min_seen : i32 = arr[right];
        let mut left : usize = right;
        for i in (0..arr.len()).rev() {
            min_seen = min(min_seen, arr[i]);
            if arr[i] > min_seen {
                left = i;
            }
        }

        (left, right)
    }

    /// O(n^2) time, O(1) space
    pub fn get_smallest_window_brute_force(arr : &Vec<i32>) -> (usize, usize) {
        if arr.len() <= 1 {
            // add errors later...
            return (0, 0);
        }

        let mut result : Option<(usize, usize)> = None;
        for i in 0..arr.len() {
            let left = i;
            let mut right : Option<usize> = None;
            for j in i + 1..arr.len() {
                if arr[j] < arr[i] {
                    right = Some(j);
                }
            }

            match right {
                Some(x) => match result {
                    Some(res) => if (res.1 - res.0) > (x - left) {
                                    result = Some((left, x))
                                },
                    None => result = Some((left, x)),
                },
                None => (),
            }
        }

        result.unwrap_or((0, 0))
    }

    /// O(n log n) time O(n) space
    pub fn get_smallest_window_using_sorting(arr : &Vec<i32>) -> (usize, usize) {
        if arr.len() <= 1 {
            // add errors later...
            return (0, 0);
        }

        // copy the vec and sort it
        let mut sorted_arr : Vec<i32> = arr.to_vec();
        sorted_arr.sort();

        // find the left hand side
        let mut left : Option<usize> = None;
        for i in 0..arr.len() {
            if arr[i] != sorted_arr[i] {
                left = Some(i);
                break;
            }
        }

        match left {
            Some(x) => {
                for i in x + 1..arr.len() {
                    if arr[i] == sorted_arr[i] {
                        return (x, i - 1);
                    }
                }
                (x, x)
            },
            None => (0, 0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn test_functions(input : Vec<i32>, start_index : usize, end_index : usize) {
            let expected : (usize, usize) = (start_index, end_index);
            assert_eq!(get_smallest_window_traverse_both_ends(&input), expected);
            assert_eq!(get_smallest_window_brute_force(&input), expected);
            assert_eq!(get_smallest_window_using_sorting(&input), expected);
        }

        #[test]
        fn test_empty_arr() {
            let arr : Vec<i32> = Vec::new();
            test_functions(arr, /*start_index=*/0, /*end_index=*/0);
        }

        #[test]
        fn test_size_one_arr() {
            let arr : Vec<i32> = vec![1];
            test_functions(arr, /*start_index=*/0, /*end_index=*/0);
        }

        #[test]
        fn test_size_all_zeroes() {
            let arr : Vec<i32> = vec![0, 0, 0];
            test_functions(arr, /*start_index=*/0, /*end_index=*/0);
        }

        #[test]
        fn test_actual() {
            let arr : Vec<i32> = vec![3, 7, 5, 6, 9];
            test_functions(arr, /*start_index=*/1, /*end_index=*/3);
        }
    }
}
