/// Given an array of numbers, find the maximum sum of any
/// contiguous subarray of the array.
/// Includes wrap around array capabilities as well
mod max_subarray {
    use std::cmp::{max, min};

    /// O(n) time O(1) space
    pub fn get_max_subarray(arr : &Vec<i32>) -> i32 {
        if arr.len() == 0 {
            return 0;
        }

        let mut max_seen : i32 = 0;
        let mut current_sum : i32 = 0;
        // Iterate over the array and look at the max possible subarray ending at the element
        // for each index we could include or exclude it.
        for i in arr {
            current_sum = max(current_sum + i, *i);
            max_seen = max(max_seen, current_sum)
        }

        max_seen
    }

    /// O(n^2) time O(1) space
    pub fn get_max_subarray_brute_force(arr : &Vec<i32>) -> i32 {
        if arr.len() == 0 {
            return 0;
        }

        if arr.len() == 1 {
            return arr[0];
        }

        let mut max_seen : i32 = 0;
        for i in 0..arr.len() {
            let mut current_sum : i32 = arr[i];
            for j in i + 1..arr.len() {
                current_sum = current_sum + arr[j];
                max_seen = max(max_seen, current_sum)
            }
        }

        max_seen
    }

    /// O(n) time O(1) space
    pub fn get_max_subarray_wrap_around(arr : &Vec<i32>) -> i32 {
        if arr.len() == 0 {
            return 0;
        }

        // Get the max of the non-wrapped around subarray
        let max_seen = get_max_subarray(arr);

        // Find the min subarray, and subtract from array total
        // to find max of wraparound subarray
        // Because for any subarray that wraps around, there must be
        // some contiguous elements that must be excluded, this is the min subarray
        let mut min_subarray_sum = 0;
        let mut current_sum = 0;
        for i in arr {
            current_sum = min(current_sum + i, *i);
            min_subarray_sum  = min(current_sum, min_subarray_sum )
        }

        let array_sum : i32 = arr.iter()
                                 .fold(0i32, |sum, x| sum + x);

        max(max_seen, array_sum - min_subarray_sum)
    }

    /// O(n^2) time O(1) space
    pub fn get_max_subarray_brute_force_wrap_around(arr : &Vec<i32>) -> i32 {
        if arr.len() == 0 {
            return 0;
        }

        if arr.len() == 1 {
            return arr[0];
        }

        let mut max_seen : i32 = 0;
        for i in 0..arr.len() {
            let mut current_sum : i32 = 0;
            for j in 0..arr.len() {
                if j == i {
                    continue;
                }
                current_sum = current_sum + arr[j];
                max_seen = max(max_seen, current_sum)
            }
        }

        max_seen
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn test_functions(input : Vec<i32>, expected_nonwraparound : i32, expected_wraparound : i32) {
            assert_eq!(get_max_subarray(&input), expected_nonwraparound);
            assert_eq!(get_max_subarray_brute_force(&input), expected_nonwraparound);
            assert_eq!(get_max_subarray_wrap_around(&input), expected_wraparound);
            assert_eq!(get_max_subarray_brute_force_wrap_around(&input), expected_wraparound);
        }

        #[test]
        fn test_empty_arr() {
            let arr : Vec<i32> = Vec::new();
            test_functions(arr, /*expected_nonwraparound=*/0, /*expected_wraparound=*/0);
        }

        #[test]
        fn test_size_one_arr() {
            let arr : Vec<i32> = vec![1];
            test_functions(arr, /*expected_nonwraparound=*/1, /*expected_wraparound=*/1);
        }

        #[test]
        fn test_size_all_zeroes() {
            let arr : Vec<i32> = vec![0, 0, 0];
            test_functions(arr, /*expected_nonwraparound=*/0, /*expected_wraparound=*/0);
        }

        #[test]
        fn test_all_negative() {
            let arr : Vec<i32> = vec![-5, -1, -9, -8];
            test_functions(arr, /*expected_nonwraparound=*/0, /*expected_wraparound=*/0);
        }

        #[test]
        fn test_actual() {
            let arr : Vec<i32> = vec![34, -50, 42, 14, -5, 86];
            test_functions(arr, /*expected_nonwraparound=*/137, /*expected_wraparound=*/171);
        }

        #[test]
        fn test_actual_2() {
            let arr : Vec<i32> = vec![8, -1, 3, 4];
            test_functions(arr, /*expected_nonwraparound=*/14, /*expected_wraparound=*/15);
        }
    }
}
