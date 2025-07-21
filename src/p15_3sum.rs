use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut answer: HashSet<Vec<i32>> = HashSet::new();
        let mut nums = nums;
        nums.sort();
        let mut last = nums[0] - 1;
        let mut i = 0;
        while i < (nums.len() - 2) {
            if last == nums[i] {
                i += 1;
                continue;
            }
            last = nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    let mut vector = vec![nums[i], nums[j], nums[k]];
                    vector.sort();
                    answer.insert(vector);
                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
            i += 1
        }
        let answer = answer.into_iter().collect();
        answer
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn test_15() {
        let set_vector = Solution::three_sum(vec![-1,0,1,2,-1,-4]);
        let set: HashSet<Vec<i32>> = set_vector.into_iter().collect();
        assert_eq!(set, HashSet::from([vec![-1, 0, 1], vec![-1, -1, 2]]));
    }
}