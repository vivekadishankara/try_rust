#![allow(dead_code)]
struct Solution;

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut copy_x = x;
        let mut reverse = 0;
        while copy_x > 0 {
            let digit = copy_x % 10;
            reverse = reverse * 10 + digit;
            copy_x = copy_x / 10;
        }
        x == reverse
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_13(){
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}