use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut number_map: HashMap<i32, i32> = HashMap::new();

        for (idx, a_num) in nums.iter().enumerate() {
            if let Some(index) = number_map.get(&(target - a_num)) {
                return vec![*index as i32, idx as i32];
            } else {
                number_map.insert(*a_num, idx as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_13(){
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    }
}