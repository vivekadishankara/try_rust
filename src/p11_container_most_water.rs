#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;

        while left < right {
            let mut area = right - left;
            let curr_height = if height[left] < height[right] {
                left += 1;
                height[left - 1]
            } else {
                right -= 1;
                height[right + 1]
            };
            area *= curr_height as usize;

            if area > result {
                result = area;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_11(){
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![1,1]), 1);
    }
}