/// Given an array of integers, return a new array where each element
/// in the new array is the number of smaller elements to the right of that element
/// in the original array
/// Assume distinct values
mod num_smaller_elements_to_the_right {
    /// O(n) space
    // pub fn num_smaller_elements_to_the_right(arr: &Vec<i32>) -> Vec<i32> {
    //
    // }

    /// O(n^2) time O(n) space
    pub fn num_smaller_elements_to_the_right_brute_force(arr: &Vec<i32>) -> Vec<i32> {
        if arr.len() == 0 {
            return Vec::new();
        }

        let mut result : Vec<i32> = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            let mut num_smaller = 0;
            for j in i+1..arr.len() {
                if arr[j] < arr[i] {
                    num_smaller += 1;
                }
            }

            result.push(num_smaller);
        }

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn test_functions(input : Vec<i32>, expected : Vec<i32>) {
            //assert_eq!(num_smaller_elements_to_the_right(&input), expected);
            assert_eq!(num_smaller_elements_to_the_right_brute_force(&input), expected);
        }

        #[test]
        fn test_empty_arr() {
            let arr : Vec<i32> = Vec::new();
            let expected : Vec<i32> = vec![];
            test_functions(arr, expected);
        }

        #[test]
        fn test_size_one_arr() {
            let arr : Vec<i32> = vec![1];
            let expected : Vec<i32> = vec![0];
            test_functions(arr, expected);
        }

        #[test]
        fn test_size_all_zeroes() {
            let arr : Vec<i32> = vec![0, 0, 0];
            let expected : Vec<i32> = vec![0, 0, 0];
            test_functions(arr, expected);
        }

        #[test]
        fn test_actual() {
            let arr : Vec<i32> = vec![3, 4, 9, 6, 1];
            let expected : Vec<i32> = vec![1, 1, 2, 1, 0];
            test_functions(arr, expected);
        }
    }
}
