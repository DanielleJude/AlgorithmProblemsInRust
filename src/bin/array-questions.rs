
/// Given an array of integers, return a new array s.t. each element at index
/// i of the new array is the product of all the numbers in the original array
/// except the one at i
mod array_products {
    /// Use both prefix and suffix product arrays
    /// O(n) time, O(n) space.
    pub fn get_product_of_all_other_elements_no_division(arr : &Vec<i32>) -> Vec<i32> {
        if arr.len() == 0 {
            return Vec::new();
        }

        if arr.len() == 1 {
            return vec![1]
        }

        // Create a prefix products array where each i is the product of all # before and including i
        let mut prefix_products : Vec<i32> = Vec::with_capacity(arr.len());
        let mut prev_number = 1;
        for i in 0..arr.len() {
            let num = arr[i];
            prefix_products.push(num * prev_number);
            prev_number = prev_number * num;
        }

        // Create a suffix products array where each i is the product of all # after and including i
        let mut suffix_products : Vec<i32> = vec![1; arr.len()];
        prev_number = 1;
        for i in (0..arr.len()).rev() {
            let num = arr[i];
            suffix_products[i] = num * prev_number;
            prev_number = prev_number * num;
        }

        let mut result = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            if i == arr.len() - 1 {
                result.push(prefix_products[i - 1])
            }
            else if i == 0 {
                result.push(suffix_products[i + 1])
            } else {
                result.push(prefix_products[i - 1] * suffix_products[i + 1]);
            }
        }

        result
    }

    /// Assume we are not allowed to use the division operator
    /// O(n^2) time, O(n) space (if in place, would be O(1))
    pub fn get_product_of_all_other_elements_brute_force(arr : &Vec<i32>)  -> Vec<i32> {
        if arr.len() == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            result.push(1);
            for j in 0..arr.len() {
                if i != j {
                    result[i] = result[i] * arr[j];
                }
            }
        }

        result
    }

    /// O(n) time, O(n) space (if in place, would be O(1))
    pub fn get_product_of_all_other_elements(arr : &Vec<i32>)  -> Vec<i32> {
        if arr.len() == 0 {
            return Vec::new();
        }

        let entire_product = arr.iter()
                                .fold(1, |accumulation, num| num * accumulation);

        let result = arr.iter()
                        .map(|num| if *num == 0 {0} else {entire_product / *num})
                        .collect();

        result
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_empty_arr() {
            let arr : Vec<i32> = Vec::new();
            let expected : Vec<i32> = Vec::new();
            assert_eq!(get_product_of_all_other_elements(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_brute_force(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_no_division(&arr), expected);
        }

        #[test]
        fn test_size_one_arr() {
            let arr : Vec<i32> = vec![5];
            let expected : Vec<i32> = vec![1];
            assert_eq!(get_product_of_all_other_elements(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_brute_force(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_no_division(&arr), expected);
        }

        #[test]
        fn test_all_zeroes() {
            let arr : Vec<i32> = vec![0, 0, 0];
            let expected : Vec<i32> = vec![0, 0, 0];
            assert_eq!(get_product_of_all_other_elements(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_brute_force(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_no_division(&arr), expected);
        }

        #[test]
        fn test_one_to_five() {
            let arr : Vec<i32> = vec![1, 2, 3, 4, 5];
            let expected : Vec<i32> = vec![120, 60, 40, 30, 24];
            assert_eq!(get_product_of_all_other_elements(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_brute_force(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_no_division(&arr), expected);
        }

        #[test]
        fn test_three_to_one() {
            let arr : Vec<i32> = vec![3, 2, 1];
            let expected : Vec<i32> = vec![2, 3, 6];
            assert_eq!(get_product_of_all_other_elements(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_brute_force(&arr), expected);
            assert_eq!(get_product_of_all_other_elements_no_division(&arr), expected);
        }

    }
}
