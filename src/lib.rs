// 1. Two Sum
// Given an array of integers nums and an integer target, 
// return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, 
// and you may not use the same element twice.
// You can return the answer in any order.

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }

    vec![] 
}


#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add() {
        assert_eq!(two_sum(vec![1,2,3,4,5], 9), vec![3,4]);
        assert_eq!(two_sum(vec![2,5,6,4], 10), vec![2,3]);
        assert_eq!(two_sum(vec![1,2,3,4,5], 3), vec![0,1]);
    }
}


